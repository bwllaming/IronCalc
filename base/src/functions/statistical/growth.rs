use crate::{
    calc_result::CalcResult,
    expressions::{parser::ArrayNode, parser::Node, token::Error, types::CellReferenceIndex},
    model::Model,
};

#[derive(Clone)]
struct NumericMatrix {
    rows: usize,
    columns: usize,
    values: Vec<Option<f64>>,
}

impl NumericMatrix {
    fn len(&self) -> usize {
        self.values.len()
    }

    fn is_vector(&self) -> bool {
        self.rows == 1 || self.columns == 1
    }
}

impl<'a> Model<'a> {
    // GROWTH(known_y's, [known_x's], [new_x's], [const])
    //
    // This fork extension covers Excel-compatible one-dimensional exponential
    // regression, returning a dynamic array shaped like new_x's.
    pub(crate) fn fn_growth(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() || args.len() > 4 {
            return CalcResult::new_args_number_error(cell);
        }

        let known_y = match self.growth_numeric_matrix(&args[0], cell) {
            Ok(matrix) => matrix,
            Err(error) => return error,
        };
        if !known_y.is_vector() || known_y.len() == 0 {
            return CalcResult::new_error(
                Error::VALUE,
                cell,
                "GROWTH requires a one-dimensional known_y range".to_string(),
            );
        }

        let known_x = if args.get(1).is_none() || matches!(args.get(1), Some(Node::EmptyArgKind)) {
            growth_default_known_x(&known_y)
        } else {
            match self.growth_numeric_matrix(&args[1], cell) {
                Ok(matrix) => matrix,
                Err(error) => return error,
            }
        };
        if !known_x.is_vector() || known_x.len() != known_y.len() {
            return CalcResult::new_error(
                Error::VALUE,
                cell,
                "GROWTH known_x must be a one-dimensional range with the same length as known_y"
                    .to_string(),
            );
        }

        let new_x = if args.get(2).is_none() || matches!(args.get(2), Some(Node::EmptyArgKind)) {
            known_x.clone()
        } else {
            match self.growth_numeric_matrix(&args[2], cell) {
                Ok(matrix) => matrix,
                Err(error) => return error,
            }
        };
        if !new_x.is_vector() {
            return CalcResult::new_error(
                Error::VALUE,
                cell,
                "GROWTH currently supports one-dimensional new_x ranges".to_string(),
            );
        }

        let include_intercept =
            if args.get(3).is_none() || matches!(args.get(3), Some(Node::EmptyArgKind)) {
                true
            } else {
                match self.get_boolean(&args[3], cell) {
                    Ok(value) => value,
                    Err(error) => return error,
                }
            };

        let (intercept, slope) =
            match growth_fit_one_variable(&known_y.values, &known_x.values, include_intercept) {
                Ok(coefficients) => coefficients,
                Err(error) => {
                    return CalcResult::new_error(error, cell, "Invalid GROWTH inputs".to_string())
                }
            };

        let mut output = Vec::with_capacity(new_x.rows);
        for row in 0..new_x.rows {
            let mut output_row = Vec::with_capacity(new_x.columns);
            for column in 0..new_x.columns {
                let index = row * new_x.columns + column;
                let Some(x) = new_x.values[index] else {
                    output_row.push(ArrayNode::Error(Error::VALUE));
                    continue;
                };
                let value = (intercept + slope * x).exp();
                if value.is_finite() {
                    output_row.push(ArrayNode::Number(value));
                } else {
                    output_row.push(ArrayNode::Error(Error::NUM));
                }
            }
            output.push(output_row);
        }

        CalcResult::Array(output)
    }

    fn growth_numeric_matrix(
        &mut self,
        node: &Node,
        cell: CellReferenceIndex,
    ) -> Result<NumericMatrix, CalcResult> {
        let array = self.eval_to_array(node, cell)?;
        let rows = array.len();
        let columns = array.first().map_or(0, Vec::len);
        if rows == 0 || columns == 0 || array.iter().any(|row| row.len() != columns) {
            return Err(CalcResult::new_error(
                Error::VALUE,
                cell,
                "GROWTH requires a rectangular input range".to_string(),
            ));
        }

        let mut values = Vec::with_capacity(rows * columns);
        for row in array {
            for item in row {
                match item {
                    ArrayNode::Number(value) => values.push(Some(value)),
                    ArrayNode::Empty | ArrayNode::String(_) | ArrayNode::Boolean(_) => {
                        values.push(None)
                    }
                    ArrayNode::Error(error) => {
                        return Err(CalcResult::new_error(
                            error,
                            cell,
                            "GROWTH input contains an error".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(NumericMatrix {
            rows,
            columns,
            values,
        })
    }
}

fn growth_default_known_x(known_y: &NumericMatrix) -> NumericMatrix {
    let values = (1..=known_y.len())
        .map(|index| Some(index as f64))
        .collect();
    NumericMatrix {
        rows: known_y.rows,
        columns: known_y.columns,
        values,
    }
}

fn growth_fit_one_variable(
    known_y: &[Option<f64>],
    known_x: &[Option<f64>],
    include_intercept: bool,
) -> Result<(f64, f64), Error> {
    let mut pairs = Vec::new();
    for (y, x) in known_y.iter().zip(known_x.iter()) {
        let (Some(y), Some(x)) = (y, x) else {
            continue;
        };
        if *y <= 0.0 {
            return Err(Error::NUM);
        }
        pairs.push((*x, y.ln()));
    }
    if pairs.len() < 2 {
        return Err(Error::DIV);
    }

    if include_intercept {
        let n = pairs.len() as f64;
        let sum_x: f64 = pairs.iter().map(|(x, _)| x).sum();
        let sum_y: f64 = pairs.iter().map(|(_, y)| y).sum();
        let sum_x2: f64 = pairs.iter().map(|(x, _)| x * x).sum();
        let sum_xy: f64 = pairs.iter().map(|(x, y)| x * y).sum();
        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator == 0.0 || !denominator.is_finite() {
            return Err(Error::DIV);
        }
        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        let intercept = (sum_y - slope * sum_x) / n;
        if !slope.is_finite() || !intercept.is_finite() {
            return Err(Error::NUM);
        }
        Ok((intercept, slope))
    } else {
        let sum_x2: f64 = pairs.iter().map(|(x, _)| x * x).sum();
        if sum_x2 == 0.0 || !sum_x2.is_finite() {
            return Err(Error::DIV);
        }
        let sum_xy: f64 = pairs.iter().map(|(x, y)| x * y).sum();
        let slope = sum_xy / sum_x2;
        if !slope.is_finite() {
            return Err(Error::NUM);
        }
        Ok((0.0, slope))
    }
}
