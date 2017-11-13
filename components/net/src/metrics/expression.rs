/*(avg by (instance)
(irate
(node_cpu
{mode="system",instance="127.0.0.1:10252",rioos_assembly_id="827188121291489280"}[5m]
)
)
* 100
)*/
use std::fmt;

pub enum Functions {
    Avg(AvgInfo),
}

pub enum Operators {
    NoOp(IRateInfo),
    IRate(IRateInfo),
}


pub struct AvgInfo {
    pub operator: Operators,
}

pub struct IRateInfo {
    pub labels: Vec<String>,
    pub metric: String,
    pub last_x_minutes: Option<String>,
}

pub struct MetricQuery {
    pub functions: Functions,
    pub by: String,
}

pub struct MetricQueryBuilder(MetricQuery);

impl MetricQueryBuilder {
    pub fn new(query: MetricQuery) -> MetricQueryBuilder {
        MetricQueryBuilder(query)
    }
}

impl fmt::Display for MetricQueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let q = &self.0;
        let b = format!("avg by ({})", q.by);
        let msg = format!("{} {}", b, q.functions);
        write!(f, "{}", msg)
    }
}


impl fmt::Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Functions::Avg(ref a) => format!("({})", a.operator),
        };
        write!(f, "{}", msg)
    }
}


impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Operators::IRate(ref i) => {
                let s: String = i.labels.clone().into_iter().collect();
                format!(
                    "irate({}{}{}{}{:?})",
                    i.metric,
                    "{",
                    s,
                    "}",
                    i.last_x_minutes,
                )
            }
            Operators::NoOp(ref i) => {
                let s: String = i.labels.clone().into_iter().collect();
                format!(
                    "{}{}{}{}{:?}",
                    i.metric,
                    "{",
                    s,
                    "}",
                    i.last_x_minutes,
                )
            }
        };
        write!(f, "{}", msg)
    }
}
