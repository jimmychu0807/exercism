use std::collections::HashSet;

type Point = (usize, usize);
type Points = HashSet<Point>;
type Rectangle = (Point, Point, Point, Point);

pub fn count(lines: &[&str]) -> u32 {
	let mut points: Points = HashSet::new();
	let mut dashes: Points = HashSet::new();
	let mut strokes: Points = HashSet::new();

	// read the coordinate point of "+"
	for (row, line) in lines.iter().enumerate() {
		for (col, chr) in line.chars().enumerate() {
			match chr {
				'+' => points.insert((row, col)),
				'-' => dashes.insert((row, col)),
				'|' => strokes.insert((row, col)),
				' ' => true,
				_ => panic!("unexpected char"),
			};
		}
	}

	if points.len() < 4 {
		return 0;
	}

	let rectangles = find_rectangles(&points, &dashes, &strokes);
	rectangles.len() as u32
}

fn find_rectangles(points: &Points, dashes: &Points, strokes: &Points) -> HashSet<Rectangle> {
	points.iter().fold(HashSet::new(), |mut acc, point| {
		let round = rec_find_rectangles(points, dashes, strokes, &[point]);
		acc.extend(&round);
		acc
	})
}

fn rec_find_rectangles(
	points: &Points,
	dashes: &Points,
	strokes: &Points,
	part: &[&Point],
) -> HashSet<Rectangle> {
	if part.len() == 1 {
		// scan downward
		let focal = part[0];

		return points
			.iter()
			.filter(|pt| {
				!part.contains(pt)
					&& pt.1 == focal.1
					&& pt.0 > focal.0
					&& is_connected(points, dashes, strokes, focal, pt)
			})
			.fold(HashSet::new(), |mut acc, pt| {
				acc.extend(&rec_find_rectangles(points, dashes, strokes, &[part[0], pt]));
				acc
			});
	} else if part.len() == 2 {
		// scan rightward
		let focal = part[1];

		return points
			.iter()
			.filter(|pt| {
				!part.contains(pt)
					&& pt.0 == focal.0
					&& pt.1 > focal.1
					&& is_connected(points, dashes, strokes, focal, pt)
			})
			.fold(HashSet::new(), |mut acc, pt| {
				acc.extend(&rec_find_rectangles(points, dashes, strokes, &[part[0], part[1], pt]));
				acc
			});
	} else if part.len() == 3 {
		// terminate condition
		// search for the last point
		let forth_point = (part[0].0, part[2].1);
		let mut res = HashSet::new();
		if points.contains(&forth_point)
			&& is_connected(points, dashes, strokes, part[0], &forth_point)
			&& is_connected(points, dashes, strokes, part[2], &forth_point)
		{
			let rect: Rectangle = (*part[0], *part[1], *part[2], forth_point);
			res.insert(rect);
		}
		return res;
	}

	panic!("shouldn't reach this point");
}

fn is_connected(
	points: &Points,
	dashes: &Points,
	strokes: &Points,
	pt1: &Point,
	pt2: &Point,
) -> bool {
	if pt1 == pt2 {
		panic!("not expecting the same point be passed in");
	}

	if pt1.0 != pt2.0 && pt1.1 != pt2.1 {
		return false;
	}

	if pt1.0 == pt2.0 {
		// These two points are having the same row coordinate and are horizontally-aligned.
		// should have a dash in-between all cells
		let min_col = pt1.1.min(pt2.1) + 1;
		let max_col = pt1.1.max(pt2.1) - 1;

		if min_col > max_col {
			return true;
		}

		for col in min_col..=max_col {
			if !dashes.contains(&(pt1.0, col)) && !points.contains(&(pt1.0, col)) {
				return false;
			}
		}
	} else {
		// These two points are having the same col coordinate and are vertically-aligned.
		// should have a stroke line in-between all cells
		let min_row = pt1.0.min(pt2.0) + 1;
		let max_row = pt1.0.max(pt2.0) - 1;

		if min_row > max_row {
			return true;
		}

		for row in min_row..=max_row {
			if !strokes.contains(&(row, pt1.1)) && !points.contains(&(row, pt1.1)) {
				return false;
			}
		}
	}

	true
}
