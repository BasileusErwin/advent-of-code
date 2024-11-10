let read_file filename =
  let channel = open_in filename in
  let rec read_lines acc =
    match input_line channel with
    | line -> read_lines (line :: acc)
    | exception End_of_file -> List.rev acc
  in
  let lines =
    try read_lines []
    with e ->
      close_in_noerr channel;
      raise e
  in
  close_in channel;
  lines

let area h w l = 2 * ((l * w) + (w * h) + (l * h))

(* Calculate the wrapping paper needed including extra for the smallest side *)
let add_extra_to_area h w l =
  let side1 = l * w and side2 = w * h and side3 = h * l in
  let smallest_side = min side1 (min side2 side3) in
  area h w l + smallest_side

(* Split a line of the form "LxWxH" into a list of integers *)
let split_numbers line = String.split_on_char 'x' line |> List.map int_of_string

(** Split all input lines into lists of integers representing dimensions *)
let split_inputs lines = List.map split_numbers lines

(* Calculate the total wrapping paper needed for all boxes (Part 1) *)
let part_1 values =
  List.fold_left
    (fun acc value ->
      match value with [ h; w; l ] -> acc + add_extra_to_area h w l | _ -> acc)
    0 values

(* Calculate the ribbon length needed for a box *)
let ribbon_length (values : int list list) : int  =
  match List.sort compare values with
  | side1 :: side2 :: _ -> (
      let smallest_perimeter = 2 * (side1 + side2) in
      match values with
      | [ h; w; l ] -> smallest_perimeter + (h * w * l)
      | _ -> 0)
  | _ -> 0

(* Calculate the total ribbon length needed for all boxes (Part 2) *)
let part_2 (values : int list list ) : int =
  List.fold_left (fun acc value -> acc + ribbon_length value) 0 values

let () =
  let filename : string = "input.txt" in
  let lines : string list = read_file filename in
  let values : int list list = split_inputs lines in
  Printf.printf "Part 1: %d\n" (part_1 values);
  Printf.printf "Part 2: %d\n" (part_2 values)
