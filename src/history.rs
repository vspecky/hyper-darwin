use crate::connection::Connection;
use crate::node::Node;

use std::vec::Vec;

pub struct History {
    pub conn_history: Vec<HistConnection>,
    next_node_innov: u32,
    next_conn_innov: u32,
}

pub struct HistConnection {
    innov: u32,
    from: u32,
    to: u32,
}

pub struct NodeMut {
    pub node: u32,
    pub in_conn: u32,
    pub out_conn: u32,
}

impl HistConnection {
    fn new(innov: u32, from: u32, to: u32) -> Self {
        Self { innov, from, to }
    }
}

impl NodeMut {
    fn new(node: u32, in_conn: u32, out_conn: u32) -> Self {
        Self {
            node,
            in_conn,
            out_conn,
        }
    }
}

impl History {
    pub fn new(inputs: u32, outputs: u32) -> Self {
        let mut hist = Self {
            conn_history: Vec::with_capacity(((inputs + 1) * outputs + 1) as usize),
            next_node_innov: inputs + outputs + 2,
            next_conn_innov: (inputs + 1) * outputs + 1,
        };

        let mut innov = 1;
        for inp in 1..=(inputs + 1) {
            for out in (inputs + 2)..(inputs + outputs + 2) {
                hist.conn_history.push(HistConnection::new(innov, inp, out));
                innov += 1;
            }
        }

        hist
    }

    pub fn mutate_conn(&mut self, from: &Node, to: &Node) -> u32 {
        match self
            .conn_history
            .iter()
            .find(|c| c.from == from.innov && c.to == to.innov)
        {
            Some(conn) => conn.innov,
            None => {
                let res = self.next_conn_innov;
                self.next_conn_innov += 1;
                self.conn_history
                    .push(HistConnection::new(res, from.innov, to.innov));
                res
            }
        }
    }

    pub fn mutate_node(&mut self, conn: &Connection) -> NodeMut {
        let from_conns = self
            .conn_history
            .iter()
            .filter(|c| c.from == conn.from)
            .collect::<Vec<&HistConnection>>();
        let to_conns = self
            .conn_history
            .iter()
            .filter(|c| c.to == conn.to)
            .collect::<Vec<&HistConnection>>();

        let mut res: Option<NodeMut> = None;

        'outer: for from_conn in from_conns {
            for to_conn in &to_conns {
                if from_conn.to == to_conn.from {
                    res = Some(NodeMut::new(from_conn.to, from_conn.innov, to_conn.innov));
                    break 'outer;
                }
            }
        }

        match res {
            Some(innov) => innov,
            None => {
                let new_node_innov = self.next_node_innov;
                self.next_node_innov += 1;
                let new_in_innov = self.next_conn_innov;
                self.next_conn_innov += 1;
                let new_in = HistConnection::new(new_in_innov, conn.from, new_node_innov);

                let new_out_innov = self.next_conn_innov;
                self.next_conn_innov += 1;
                let new_out = HistConnection::new(new_out_innov, new_node_innov, conn.to);

                self.conn_history.push(new_in);
                self.conn_history.push(new_out);

                NodeMut::new(new_node_innov, new_in_innov, new_out_innov)
            }
        }
    }
}
