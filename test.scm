(define (squarelist lst)
  (if (null? lst)
    ()
    (cons (* (car lst) (car lst)) (squarelist (cdr lst)))))
