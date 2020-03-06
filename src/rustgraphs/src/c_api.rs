use crate::graphs::ListGraph;
use ::scopeguard::defer_on_unwind;
use core::convert::TryInto;
use core::slice;
use libc::size_t;

#[repr(C)]
pub struct graph_buffer {
    array: *mut i32,
    array_length: size_t,
    node_degrees: *mut size_t,
    node_degrees_length: size_t,
}

pub fn vecvec_to_graph_buffer(graph: Vec<Vec<i32>>) -> graph_buffer {
    let array: Box<[i32]> = graph.concat().into_boxed_slice();
    let node_degrees: Vec<usize> = graph
        .iter()
        .map(|v| v.len().try_into().expect("Integer Overflow"))
        .collect();
    let node_degrees: Box<[usize]> = node_degrees.into_boxed_slice();

    graph_buffer {
        array_length: array.len().try_into().expect("Integer Overflow"),
        array: Box::into_raw(array) as _,
        node_degrees_length: node_degrees.len().try_into().expect("Integer Overflow"),
        node_degrees: Box::into_raw(node_degrees) as _,
    }
}

#[no_mangle]
pub extern "C" fn erdos(n: size_t, p: f64) -> graph_buffer {
    defer_on_unwind!({
        std::process::abort();
    });
    let graph = ListGraph::erdos(n, p);
    let v = graph.get_vec();
    vecvec_to_graph_buffer(v)
}

#[no_mangle]
pub extern "C" fn fast_erdos(n: size_t, p: f64) -> graph_buffer {
    defer_on_unwind!({
        std::process::abort();
    });
    let graph = ListGraph::fast_erdos(n, p);
    let v = graph.get_vec();
    vecvec_to_graph_buffer(v)
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
#[no_mangle]
pub unsafe extern "C" fn rust_free(buf: graph_buffer) {
    defer_on_unwind!({
        std::process::abort();
    });
    let graph_buffer {
        array,
        array_length,
        node_degrees,
        node_degrees_length,
    } = buf;
    drop::<Box<[i32]>>(Box::from_raw(slice::from_raw_parts_mut(
        array,
        array_length.try_into().expect("Integer Overflow"),
    )));
    drop::<Box<[usize]>>(Box::from_raw(slice::from_raw_parts_mut(
        node_degrees,
        node_degrees_length.try_into().expect("Integer Overflow"),
    )));
}
