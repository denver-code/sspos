(load "/lib/lisp/core.lsp")

(def (fibonacci n)
  (if (< n 2) n
    (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))

(println
  (if (nil? args) "Usage: fibonacci <num>"
    (fibonacci (string->number (car args)))))
