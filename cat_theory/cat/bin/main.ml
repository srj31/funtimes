let () = print_endline "Hello, World!"
let i = Cat.Chap1.id 40
let v = Cat.Chap1.(id << id) 40
let () = print_endline (string_of_int (i + v))
