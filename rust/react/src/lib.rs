use std::cmp::{Ord, PartialOrd};
use std::collections::BTreeMap;
use std::fmt::Debug;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputCellId(u64);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComputeCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallbackId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
	Input(InputCellId),
	Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
	NonexistentCell,
	NonexistentCallback,
}

pub struct Reactor<'a, T> {
	// Just so that the compiler doesn't complain about an unused type parameter.
	// You probably want to delete this field.
	next_id: u64,
	input_cells: BTreeMap<InputCellId, T>,

	#[allow(clippy::type_complexity)]
	compute_cells: BTreeMap<ComputeCellId, (Vec<CellId>, Box<dyn Fn(&[T]) -> T + 'a>)>,
	callbacks: BTreeMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
	dependencies: BTreeMap<ComputeCellId, Vec<CallbackId>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Debug> Reactor<'a, T> {
	pub fn new() -> Self {
		Self {
			next_id: 0,
			input_cells: BTreeMap::new(),
			compute_cells: BTreeMap::new(),
			callbacks: BTreeMap::new(),
			dependencies: BTreeMap::new(),
		}
	}

	fn get_next_id(&mut self) -> u64 {
		let ret = self.next_id;
		self.next_id += 1;
		ret
	}

	// Creates an input cell with the specified initial value, returning its ID.
	pub fn create_input(&mut self, _initial: T) -> InputCellId {
		let id = self.get_next_id();
		self.input_cells.insert(InputCellId(id), _initial);
		InputCellId(id)
	}

	// Creates a compute cell with the specified dependencies and compute function.
	// The compute function is expected to take in its arguments in the same order as specified in
	// `dependencies`.
	// You do not need to reject compute functions that expect more arguments than there are
	// dependencies (how would you check for this, anyway?).
	//
	// If any dependency doesn't exist, returns an Err with that nonexistent dependency.
	// (If multiple dependencies do not exist, exactly which one is returned is not defined and
	// will not be tested)
	//
	// Notice that there is no way to *remove* a cell.
	// This means that you may assume, without checking, that if the dependencies exist at creation
	// time they will continue to exist as long as the Reactor exists.
	pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
		&mut self,
		_dependencies: &[CellId],
		_compute_func: F,
	) -> Result<ComputeCellId, CellId> {
		//1. check that all dependent cells exist
		for dep in _dependencies.iter() {
			match dep {
				CellId::Input(cell_id) if !self.input_cells.contains_key(cell_id) => {
					return Err(*dep);
				}
				CellId::Compute(cell_id) if !self.compute_cells.contains_key(cell_id) => {
					return Err(*dep);
				}
				_ => {}
			}
		}

		//2. create the compute cell storing the dependencies ahd compute function
		let id = ComputeCellId(self.get_next_id());

		self.compute_cells.insert(id, (_dependencies.to_vec(), Box::new(_compute_func)));

		//3. return the ComputeCellId
		Ok(id)
	}

	// Retrieves the current value of the cell, or None if the cell does not exist.
	//
	// You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
	// and have a `value(&self)` method on `Cell`.
	//
	// It turns out this introduces a significant amount of extra complexity to this exercise.
	// We chose not to cover this here, since this exercise is probably enough work as-is.
	pub fn value(&self, id: CellId) -> Option<T> {
		match id {
			CellId::Input(id) => self.input_cells.get(&id).copied(),
			CellId::Compute(id) => {
				if let Some((dependencies, func)) = self.compute_cells.get(&id) {
					let dep_vals: Vec<_> =
						dependencies.iter().map(|cell_id| self.value(*cell_id).unwrap()).collect();
					Some(func(&dep_vals))
				} else {
					None
				}
			}
		}
	}

	// Sets the value of the specified input cell.
	//
	// Returns false if the cell does not exist.
	//
	// Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
	// a `set_value(&mut self, new_value: T)` method on `Cell`.
	//
	// As before, that turned out to add too much extra complexity.
	pub fn set_value(&mut self, input_id: InputCellId, _new_value: T) -> bool {
		if !self.input_cells.contains_key(&input_id) {
			return false;
		}

		// 1. Find the potentially affected cells
		//   loop thru all compute_cells and see which one depends on input_cell
		//   need to loop multiple time, when no more compute cells are added, goto the next stage
		let mut affected_cells: Vec<&ComputeCellId> = Vec::new();

		println!("loop begin");

		loop {
			let mut round_affected = self.compute_cells.keys()
				.filter_map(|k| {
					let cell = self.compute_cells.get(k).unwrap();
					// TODO: moddify the following if to check the affected_cells
					//   check intersection of cell.0 and affected_cells
					if cell.0.contains(&CellId::Input(input_id)) { Some (k) } else { None }
				})
				.collect::<Vec<_>>();

			if round_affected.is_empty() { break; }
			affected_cells.append(&mut round_affected);

			break;
		}

		println!("affected_cells: {:?}", affected_cells);

		let prev_result: Vec<_> = affected_cells.iter().filter_map(|compute_id| self.value(CellId::Compute(**compute_id))).collect();

		// 2. Set the value
		self.input_cells.insert(input_id, _new_value);

		let curr_result: Vec<_> = affected_cells.iter().filter_map(|compute_id| self.value(CellId::Compute(**compute_id))).collect();

		// 3. Filter out affected cells with no delta
		let affected_cells_with_delta: Vec<_> = affected_cells.iter().enumerate()
			.filter(|(idx, _)| prev_result[*idx] != curr_result[*idx])
			.map(|(_, compute_id)| compute_id)
			.collect();

		println!("prev_result: {:?}", prev_result);
		println!("curr_result: {:?}", curr_result);
		println!("affected_cells_with_delta: {:?}", affected_cells_with_delta);

		// 4. for all compute_cells, see which one trigger any callback
		let callbacks: Vec<(ComputeCellId, CallbackId)> = affected_cells_with_delta.into_iter()
			.filter_map(|compute_id| self.dependencies.get(compute_id).map(|cb_vec| (compute_id, cb_vec)))
			.inspect(|(compute_id, cb_vec)| {
				println!("compute_id: {:?}, cb_vec: {:?}", compute_id, cb_vec);
			})
			.fold(Vec::new(), |mut acc, (compute_id, cb_vec)| {
				for cb_id in cb_vec {
					// we always want to call the callback last and once only. So
					//   remove it from the queue and push it to the back.
					if let Some(pos) = acc.iter().position(|(_, inside_cb_id)| inside_cb_id == cb_id) {
						acc.remove(pos);
					}

					println!("compute_id: {:?}, cb_id: {:?}", compute_id, cb_id);
					acc.push((**compute_id, *cb_id));
				}
				acc
			});

		println!("callbacks: {:?}", callbacks);

		// 5. Perform the callback
		callbacks.into_iter().for_each(|(compute_id, cb_id)| {
			let val = self.value(CellId::Compute(compute_id)).unwrap();
			if let Some(func) = self.callbacks.get_mut(&cb_id) {
				func(val);
			}
		});

		true
	}

	// Adds a callback to the specified compute cell.
	//
	// Returns the ID of the just-added callback, or None if the cell doesn't exist.
	//
	// Callbacks on input cells will not be tested.
	//
	// The semantics of callbacks (as will be tested):
	// For a single set_value call, each compute cell's callbacks should each be called:
	// * Zero times if the compute cell's value did not change as a result of the set_value call.
	// * Exactly once if the compute cell's value changed as a result of the set_value call.
	//   The value passed to the callback should be the final value of the compute cell after the
	//   set_value call.
	pub fn add_callback<F: FnMut(T) + 'a>(
		&mut self,
		compute_cell_id: ComputeCellId,
		_callback: F,
	) -> Option<CallbackId> {
		if !self.compute_cells.contains_key(&compute_cell_id) {
			return None;
		}

		let id = CallbackId(self.get_next_id());

		self.callbacks.insert(id, Box::new(_callback));

		self.dependencies.entry(compute_cell_id)
			.and_modify(|e| e.push(id))
			.or_insert(vec![id]);

		Some(id)
	}

	// Removes the specified callback, using an ID returned from add_callback.
	//
	// Returns an Err if either the cell or callback does not exist.
	//
	// A removed callback should no longer be called.
	pub fn remove_callback(
		&mut self,
		cell: ComputeCellId,
		callback: CallbackId,
	) -> Result<(), RemoveCallbackError> {
		// compute_cell or callback doesn't exist.
		if !self.compute_cells.contains_key(&cell) {
			return Err(RemoveCallbackError::NonexistentCell);
		}

		if !self.callbacks.contains_key(&callback) {
			return Err(RemoveCallbackError::NonexistentCallback);
		}

		if let Some(vec) = self.dependencies.get_mut(&cell) {
			if let Some(pos) = vec.iter().position(|&cb_id| cb_id == callback) {
				vec.remove(pos);
			} else {
				return Err(RemoveCallbackError::NonexistentCallback);
			}
		} else {
			return Err(RemoveCallbackError::NonexistentCell);
		}

		// Can't remove from self.callbacks, you don't know if other compute cells are
		// dependent to this callback.

		Ok(())
	}
}
