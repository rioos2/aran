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
    Sum(AvgInfo),
    SumDisk(AvgInfo),
}

pub enum Operators {
    NoOp(IRateInfo),
    IRate(IRateInfo),
    Sum(SumInfo),
    SumDisk(SumInfo),
}

pub struct AvgInfo {
    pub operator: Operators,
}

pub struct IRateInfo {
    pub labels: Vec<String>,
    pub metric: String,
    pub last_x_minutes: String,
}

pub struct SumInfo {
    pub labels: Vec<String>,
    pub metric: Vec<String>,
    pub total: String,
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
        let b = format!("{}", q.by);
        let msg = format!("{} {}", b, q.functions);
        write!(f, "{}", msg)
    }
}

impl fmt::Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Functions::Avg(ref a) => format!("({})", a.operator),
            Functions::Sum(ref a) => format!("{}", a.operator),
            Functions::SumDisk(ref a) => format!("{}", a.operator),
        };
        write!(f, "{}", msg)
    }
}

impl fmt::Display for Operators {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            //generate the query to average value
            Operators::IRate(ref i) => {
                let s: String = i.labels
                    .clone()
                    .into_iter()
                    .map(|x| {
                        let data = x.split("=").collect::<Vec<_>>();
                        format!("{}={}{}{}{}", data[0], '"', data[1], '"', ",")
                    })
                    .collect();
                format!("irate({}{}{}{}{})", i.metric, "{", s, "}", i.last_x_minutes,)
            }
            //generate the query to get usage of node memory
            Operators::Sum(ref i) => {
                let s: String = i.labels
                    .clone()
                    .into_iter()
                    .map(|x| {
                        let data = x.split("=").collect::<Vec<_>>();
                        format!("{}={}{}{}{}", data[0], '"', data[1], '"', ",")
                    })
                    .collect();
                let r: Vec<String> = i.metric
                    .clone()
                    .into_iter()
                    .map(|x| format!("{}({}{}{}{})", i.total, x, "{", s, "}"))
                    .collect::<Vec<_>>();
                format!(
                    "({}-{}-{})/{}({}) *100",
                    r[0],
                    r[1],
                    r[2],
                    i.total,
                    i.metric[0]
                )
            }
            //generate the query to get usage of node memory
            Operators::SumDisk(ref i) => {
                let s: String = i.labels
                    .clone()
                    .into_iter()
                    .map(|x| {
                        let data = x.split("=").collect::<Vec<_>>();
                        format!("{}={}{}{}{}", data[0], '"', data[1], '"', ",")
                    })
                    .collect();

                let r: Vec<String> = i.metric
                    .clone()
                    .into_iter()
                    .map(|x| format!("{}({}{}{}{})", i.total, x, "{", s, "}"))
                    .collect::<Vec<_>>();
                format!("({} - {})/ {}({}) *100", r[0], r[1], i.total, i.metric[0],)
            }
            Operators::NoOp(ref i) => {
                let s: String = i.labels
                    .clone()
                    .into_iter()
                    .map(|x| {
                        let data = x.split("=").collect::<Vec<_>>();
                        format!("{}={}{}{}{}", data[0], '"', data[1], '"', ",")
                    })
                    .collect();
                format!("{}{}{}{}{}", i.metric, "{", s, "}", i.last_x_minutes,)
            }
        };
        write!(f, "{}", msg)
    }
}
