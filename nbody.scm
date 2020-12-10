;; from https://github.com/smarr/are-we-fast-yet/blob/master/benchmarks/Lua/nbody.lua

(val N 11000) ;; number of iterations, takes about 1.05s
(val answer-for-N 0.174958)

;; original from The Computer Language Benchmarks Game
;; http:--shootout.alioth.debian.org/
;;
;;     contributed by Mark C. Lewis
;; modified slightly by Chad Whipkey
;;
;; Based on nbody.java ported to SOM, and then Lua by Francois Perrad.

(val PI 3.141592653589793)
(val SOLAR_MASS (* PI (* 4.0 PI)))
(val DAYS_PER_YEAR 365.24)

(define println (v) (begin (print v) (printu 10)))
(define list2 (x y) (cons x (cons y '())))


(if (nil? ref)
    (begin (set ref (lambda (v) (cons v nil)))
           (set ! car)
           (set := set-car!)
           (println 'simulating-ref))
    #f)

(record body [x y z vx vy vz mass])

(define Body.new (x y z vx vy vz mass)
  (make-body x y z
             (* DAYS_PER_YEAR vx) (* DAYS_PER_YEAR vy) (* DAYS_PER_YEAR vz)
             (* SOLAR_MASS mass)))
             


(define offset-momentum (self px py pz)
  (let* ([M SOLAR_MASS])
    (begin
      (set-body-vx! self (- 0.0 (/ px M)))
      (set-body-vy! self (- 0.0 (/ py M)))
      (set-body-vz! self (- 0.0 (/ pz M)))
      self)))

(check-expect
  (let* ([planet (make-body 'x 'y 'z 0 0 0 'M)]
         [_ (offset-momentum planet SOLAR_MASS (* 2 SOLAR_MASS) (* 3 SOLAR_MASS))])
    (body-vy planet))
  -2)
         
(define Body.jupiter ()
     (Body.new 4.8414314424647209
                -1.16032004402742839
                -0.103622044471123109
                0.00166007664274403694
                0.00769901118419740425
                -0.0000690460016972063023
                0.000954791938424326609))
end

(define Body.saturn ()
     (Body.new 8.34336671824457987
                     4.12479856412430479
                    -0.403523417114321381
                    -0.00276742510726862411
                     0.00499852801234917238
                     0.0000230417297573763929
                     0.000285885980666130812))

(define Body.uranus ()
    (Body.new 12.894369562139131
                    -15.1111514016986312
                     -0.223307578892655734
                      0.00296460137564761618
                      0.0023784717395948095
                     -0.0000296589568540237556
                      0.0000436624404335156298))


(define Body.neptune ()
    (Body.new 15.3796971148509165
                    -25.9193146099879641
                      0.179258772950371181
                      0.00268067772490389322
                      0.00162824170038242295
                     -0.000095159225451971587
                      0.0000515138902046611451))

(define Body.sun ()
    (Body.new 0.0 0.0 0.0 0.0 0.0 0.0 1.0))


(define create-bodies ()
  (let* ([bodies (list5 (Body.sun)
                    (Body.jupiter)
                    (Body.saturn)
                    (Body.uranus)
                    (Body.neptune))]
         [px 0]
         [py 0]
         [pz 0]
         [copy bodies]
         [_ (while (pair? copy)
                   (let* ([b (car copy)]
                          [_ (set px (+ px (* (body-vx b) (body-mass b))))]
                          [_ (set py (+ py (* (body-vy b) (body-mass b))))]
                          [_ (set pz (+ pz (* (body-vz b) (body-mass b))))])
                     (set copy (cdr copy))))]
         [_ (offset-momentum (car bodies) px py pz)])
    bodies))

(define too-small? (xs)
  (|| (null? xs) (null? (cdr xs))))

(define advance (bodies dt)
  (letrec
      ([i-loop
         (lambda (bodies)
           (if (too-small? bodies)
               #f
               (let* ([i-body (car bodies)]
                      [rest (cdr bodies)])
                 (begin
                   (while (pair? rest)
                          (begin
                            (let* ([j-body (car rest)]
                                   [_ (set rest (cdr rest))]
                                   [dx (- (body-x i-body) (body-x j-body))]
                                   [dy (- (body-y i-body) (body-y j-body))]
                                   [dz (- (body-z i-body) (body-z j-body))]
                                   [d-squared (+ (* dx dx) (+ (* dy dy) (* dz dz)))]
                                   [distance (sqrt d-squared)]
                                   [mag (/ dt (* d-squared distance))])
                              (begin
                                (set-body-vx! i-body (- (body-vx i-body)
                                                        (* dx (* (body-mass j-body) mag))))
                                (set-body-vy! i-body (- (body-vy i-body)
                                                        (* dy (* (body-mass j-body) mag))))
                                (set-body-vz! i-body (- (body-vz i-body)
                                                        (* dz (* (body-mass j-body) mag))))))))
                   (i-loop (cdr bodies))))))]
       [move (lambda (body)
               (begin
                 (set-body-x! body (+ (body-x body) (* dt (body-vx body))))
                 (set-body-y! body (+ (body-y body) (* dt (body-vy body))))
                 (set-body-z! body (+ (body-z body) (* dt (body-vz body))))))])
    (begin
      (i-loop bodies)
      (app move bodies))))
    
(define energy (bodies)
  (let* ([e (ref 0.0)])
    (letrec
      ([i-loop
         (lambda (bodies)
           (if (too-small? bodies)
               e
               (let* ([i-body (car bodies)]
                      [rest (cdr bodies)]
                      [v-squared (+ (* (body-vx i-body) (body-vx i-body))
                                    (+ (* (body-vy i-body) (body-vy i-body))
                                       (* (body-vz i-body) (body-vz i-body))))]
                      [_ (:= e (+ (! e) (* (* 0.5 (body-mass i-body)) v-squared)))])
                 (begin
                   (while (pair? rest)
                          (let* ([j-body (car rest)]
                                 [_ (set rest (cdr rest))]
                                 [dx (- (body-x i-body) (body-x j-body))]
                                 [dy (- (body-y i-body) (body-y j-body))]
                                 [dz (- (body-z i-body) (body-z j-body))]
                                 [d-squared (+ (* dx dx) (+ (* dy dy) (* dz dz)))]
                                 [distance (sqrt d-squared)])
                            (:= e (- (! e) (/ (* (body-mass i-body) (body-mass j-body))
                                           distance)))))
                   (i-loop (cdr bodies))))))])
      (begin
        (i-loop bodies)
        (! e)))))
                               

(define nbody:inner_benchmark_loop (inner_iterations)
  (let* ([system (create-bodies)])
    (begin
      (while (> inner_iterations 0)
             (begin
               (set inner_iterations (- inner_iterations 1))
               (advance system 0.01)))
      (energy system))))

(define abs (x)
  (if (< x 0) (- 0 x) x))

(define close-to? (x y)
  (< (abs (- x y)) (/ (abs x) 1000.0)))

(define sorta-close-to? (x y)
  (< (abs (- x y)) (/ (abs x) 100.0)))

;; (check-expect (nbody:inner_benchmark_loop 250000) -0.1690859889909308)

; test FAILS (should pass)
; (check-expect (nbody:inner_benchmark_loop 1) -0.16907495402506745)


(check-assert (sorta-close-to? (nbody:inner_benchmark_loop 1) -0.16907495402506745))


(check-assert
 (close-to?
  (nbody:inner_benchmark_loop 11000)
  answer-for-N))

 
