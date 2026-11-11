use std::sync::LazyLock;

use numeracy::matrices::{Matrix, S1, S2};

pub trait OpenGLData {
    fn memory_size(&self) -> usize;
    fn as_ptr(&self) -> *const f32;
    fn num_vertices(&self) -> usize;
    fn num_items(&self) -> usize;
    fn assert_is_data(&self);
}

impl<const NUM_ITEMS:usize> OpenGLData for Matrix<f32, 1, S1<NUM_ITEMS>> {
    fn memory_size(&self) -> usize {
        self.assert_is_data();
        let type_size = std::mem::size_of::<f32>();
        let num_items = self.array.len();
        num_items*type_size
    }
    fn as_ptr(&self) -> *const f32 {
        self.assert_is_data();
        self.as_ptr()
    }
    fn num_vertices(&self) -> usize {
        self.assert_is_data();
        1
    }
    fn num_items(&self) -> usize {
        self.assert_is_data();
        NUM_ITEMS
    }
    fn assert_is_data(&self) {
        let num_items1 = NUM_ITEMS;
        let num_items2 = self.array.len();
        assert_eq!(num_items1, num_items2);
    }
}

impl<const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize> OpenGLData for Matrix<f32, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>> {
    fn memory_size(&self) -> usize {
        let type_size = std::mem::size_of::<f32>();
        let num_items = self.array.len();
        num_items*type_size
    }
    fn as_ptr(&self) -> *const f32 {
        self.assert_is_data();
        self.as_ptr()
    }
    fn num_vertices(&self) -> usize {
        self.assert_is_data();
        NUM_VERTICES
    }
    fn num_items(&self) -> usize {
        self.assert_is_data();
        ITEMS_PER_VERTEX*NUM_VERTICES
    }
    fn assert_is_data(&self) {
        let num_items1 = ITEMS_PER_VERTEX*NUM_VERTICES;
        let num_items2 = self.array.len();
        assert_eq!(num_items1, num_items2);
        assert_eq!(NUM_VERTICES, num_items1/ITEMS_PER_VERTEX);
    }
}

impl<const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize> OpenGLData for LazyLock<Matrix<f32, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>> {
    fn memory_size(&self) -> usize {
        self.assert_is_data();
        let type_size = std::mem::size_of::<f32>();
        let num_items = self.array.len();
        num_items*type_size
    }
    fn as_ptr(&self) -> *const f32 {
        self.assert_is_data();
        self.array.as_ptr()
    }
    fn num_vertices(&self) -> usize {
        self.assert_is_data();
        NUM_VERTICES
    }
    fn num_items(&self) -> usize {
        self.assert_is_data();
        ITEMS_PER_VERTEX*NUM_VERTICES
    }
    fn assert_is_data(&self) {
        let num_items1 = ITEMS_PER_VERTEX*NUM_VERTICES;
        let num_items2 = self.array.len();
        assert_eq!(num_items1, num_items2);
        assert_eq!(NUM_VERTICES, num_items1/ITEMS_PER_VERTEX);
    }
}

impl<const ITEMS_PER_VERTEX:usize, const NUM_VERTICES:usize> OpenGLData for Vec<Matrix<f32, 2, S2<ITEMS_PER_VERTEX, NUM_VERTICES>>> {
    fn memory_size(&self) -> usize {
        let array = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat();
        self.assert_is_data();
        let array = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat();
        let type_size = std::mem::size_of::<f32>();
        let num_items = array.len();
        num_items*type_size
    }
    fn as_ptr(&self) -> *const f32 {
        self.assert_is_data();
        let array = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat();
        array.as_ptr()
    }
    fn num_vertices(&self) -> usize {
        self.assert_is_data();
        self.len()*NUM_VERTICES
    }
    fn num_items(&self) -> usize {
        self.assert_is_data();
        self.len()*ITEMS_PER_VERTEX*NUM_VERTICES
    }
    fn assert_is_data(&self) {
        let num_items1 = self.len()*ITEMS_PER_VERTEX*NUM_VERTICES;
        let num_items2 = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat().len();
        assert_eq!(num_items1, num_items2);
        assert_eq!(NUM_VERTICES, num_items1/(ITEMS_PER_VERTEX*self.len()));
    }
}

impl<const NUM_ITEMS:usize> OpenGLData for Vec<Matrix<f32, 1, S1<NUM_ITEMS>>> {
    fn memory_size(&self) -> usize {
        self.assert_is_data();
        let array = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat();
        let type_size = std::mem::size_of::<f32>();
        let num_items = array.len();
        num_items*type_size
    }
    fn as_ptr(&self) -> *const f32 {
        self.assert_is_data();
        let array = self.iter().map(|m| m.get_view_of_array()).collect::<Vec<_>>().concat();
        array.as_ptr()        
    }
    fn num_vertices(&self) -> usize {
        self.assert_is_data();
        self.len()*NUM_ITEMS
        //self.len()
    }
    fn num_items(&self) -> usize {
        self.assert_is_data();
        self.len()*NUM_ITEMS
    }
    fn assert_is_data(&self) {
        let num_items1 = NUM_ITEMS*self.len();
        let num_items2 = self.iter().map(|v| v.array.len()).collect::<Vec<usize>>().len();
        assert_eq!(num_items1, num_items2);
        assert_eq!(self.len(), num_items1/NUM_ITEMS);
    }
}