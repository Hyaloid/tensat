use crate::model::*;
use egg::{rewrite as rw, *};

// TODO each rule should also have a reverse for optimization
#[rustfmt::skip]
pub fn rules<A: Analysis<Mdl>>() -> Vec<Rewrite<Mdl, A>> { vec![
        rw!("ewadd-is-associative"            ; "(ewadd ?x (ewadd ?y ?z)) "                                             => "(ewadd (ewadd ?x ?y) ?z)"),
        rw!("ewadd-is-commutative"            ; "(ewadd ?x ?y) "                                                        => "(ewadd ?y ?x)"),
        rw!("ewmul-is-associative"            ; "(ewmul ?x (ewmul ?y ?z)) "                                             => "(ewmul (ewmul ?x ?y) ?z)"),
        rw!("ewmul-is-commutative"            ; "(ewmul ?x ?y) "                                                        => "(ewmul ?y ?x)"),
        rw!("distributivity-0"                ; "(ewmul (ewadd ?x ?y) ?z) "                                             => "(ewadd (ewmul ?x ?z) (ewmul ?y ?z))"),
        rw!("smul-is-associative"             ; "(smul (smul ?x ?y) ?w) "                                               => "(smul ?x  (smul ?y ?w))"),
        rw!("distributivity-1"                ; "(smul (ewadd ?x ?y) ?w) "                                              => "(ewadd (smul ?x ?w)  (smul ?y ?w))"),
        rw!("operator-commutativity-0"        ; "(smul (ewmul ?x ?y) ?w) "                                              => "(ewmul ?x  (smul ?y ?w))"),
        rw!("transpose-is-its-own-inverse"    ; "(transpose (transpose ?x)) "                                           => "?x"),
        rw!("operator-commutativity-1"        ; "(transpose (ewadd ?x ?y)) "                                            => "(ewadd (transpose ?x)  (transpose ?y))"),
        rw!("operator-commutativity-2"        ; "(transpose (ewmul ?x ?y)) "                                            => "(ewmul (transpose ?x)  (transpose ?y))"),
        rw!("operator-commutativity-3"        ; "(smul (transpose ?x) ?w) "                                             => "(transpose (smul ?x ?w))"),
        rw!("matmul-is-associative"           ; "(matmul ?x (matmul ?y ?z)) "                                           => "(matmul (matmul ?x ?y) ?z)"),
        rw!("matmul-is-linear-0"              ; "(smul (matmul ?x ?y) ?w) "                                             => "(matmul ?x  (smul ?y ?w))"),
        rw!("matmul-is-linear-1"              ; "(matmul ?x (ewadd ?y ?z)) "                                            => "(ewadd (matmul ?x ?y) (matmul ?x ?z))"),
        rw!("matmul-and-transpose"            ; "(transpose (matmul ?x ?y)) "                                           => "(matmul (transpose ?y)  (transpose ?x))"),
        rw!("conv-is-bilinear-0"              ; "(conv2d ?sx ?sy ?p ?c (smul ?x ?w) ?y) "                               => "(conv2d ?sx ?sy ?p ?c ?x (smul ?y ?w))"),
        rw!("conv-is-bilinear-1"              ; "(smul (conv2d ?sx ?sy ?p 0 ?x ?y) ?w) "                            => "(conv2d ?sx ?sy ?p 0 (smul ?x ?w) ?y)"),
        rw!("conv-is-bilinear-2"              ; "(conv2d ?sx ?sy ?p 0 ?x (ewadd ?y ?z)) "                           => "(ewadd (conv2d ?sx ?sy ?p 0 ?x ?y) (conv2d ?sx ?sy ?p 0 ?x ?z))"),
        rw!("conv-is-bilinear-3"              ; "(conv2d ?sx ?sy ?p 0 (ewadd ?x ?y) ?z) "                           => "(ewadd (conv2d ?sx ?sy ?p 0 ?x ?z) (conv2d ?sx ?sy ?p 0 ?y ?z))"),
        //rw!("enlarge-convolution-kernel"      ; "(conv2d ?sx ?sy 0 ?c ?x ?y) "                                      => "(conv2d ?sx ?sy 0 ?c ?x (enlarge ?kx ?ky ?y))"),
        rw!("operator-commutativity-4"        ; "(conv2d ?sx ?sy ?p 2 ?x ?y) "                                      => "(relu (conv2d ?sx ?sy ?p 0 ?x ?y))"),
        rw!("conv-with-2-applies-relu"    ; "(relu (transpose ?x)) "                                                => "(transpose (relu ?x))"),
        // rw!("pooling-by-conv.-with-Cpool"     ; "(conv2d ?sx ?sy ?p 0 ?x (Cpool ?kx ?ky)) "                              => "(poolavg ?kx ?ky ?sx ?sy ?p ?x)"),
        rw!("const_iconv-and-const_pool"      ; "(poolavg ?kx ?ky 1 1 0 (Iconv ?kx ?ky)) "                              => "(Cpool ?kx ?ky)"),
        rw!("identity-kernel"                 ; "(conv2d 1 1 0 0 ?x (Iconv ?kx ?ky)) "                               => "?x"),
        rw!("identity-matrix"                 ; "(matmul ?x   Imatmul ) "                                               => "?x"),
        rw!("ewmul-identity"                  ; "(ewmul ?x Iewmul) "                                                    => "?x"),
        rw!("split-definition-0"              ; "(split_0 ?a (concat ?a ?x ?y)) "                                       => "?x"),
        rw!("split-definition-1"              ; "(split_1 ?a (concat ?a ?x ?y)) "                                       => "?y"),
        rw!("geometry-of-concatenation"       ; "(concat 0 (concat 1 ?x ?y) (concat 1 ?z ?w)) "                         => "(concat 1 (concat 0 ?x ?z) (concat 0 ?y ?w))"),
        rw!("operator-commutativity-5"        ; "(concat ?a (smul ?x ?w) (smul ?y ?w)) "                                => "(smul (concat ?a ?x ?y) ?w)"),
        rw!("operator-commutativity-6"        ; "(concat ?a (ewadd ?x ?y) (ewadd ?z ?w)) "                              => "(ewadd (concat ?a ?x ?z) (concat ?a ?y ?w))"),
        rw!("operator-commutativity-7"        ; "(concat ?a (ewmul ?x ?y) (ewmul ?z ?w)) "                              => "(ewmul (concat ?a ?x ?z) (concat ?a ?y ?w))"),
        rw!("operator-commutativity-8"        ; "(concat ?a (relu ?x) (relu ?y)) "                                      => "(relu (concat ?a ?x ?y))"),
        rw!("concatenation-and-transpose"     ; "(concat 1 (transpose ?x) (transpose ?y)) "                             => "(transpose (concat 0 ?x ?y))"),
        rw!("concatenation-and-matrix-mul.-0" ; "(concat 1 (matmul ?x ?y) (matmul ?x ?z)) "                             => "(matmul ?x (concat 1 ?y ?z))"),
        rw!("concatenation-and-matrix-mul.-1" ; "(matmul (concat 1 ?x ?z) (concat 0 ?y ?w)) "                           => "(ewadd (matmul ?x ?y) (matmul ?z ?w))"),
        rw!("concatenation-and-conv.-0"       ; "(concat 0 (conv2d ?sx ?sy ?p ?c ?x ?z) (conv2d ?sx ?sy ?p ?c ?y ?z)) " => "(conv2d ?sx ?sy ?p ?c (concat 0 ?x ?y) ?z)"),
        rw!("concatenation-and-conv.-1"       ; "(concat 1 (conv2d ?sx ?sy ?p ?c ?x ?y) (conv2d ?sx ?sy ?p ?c ?x ?z)) " => "(conv2d ?sx ?sy ?p ?c ?x (concat 0 ?y ?z))"),
        rw!("concatenation-and-conv.-2"       ; "(conv2d ?sx ?sy ?p 0 (concat 1 ?x ?z) (concat 1 ?y ?w)) "          => "(ewadd (conv2d ?sx ?sy ?p 0 ?x ?y) (conv2d ?sx ?sy ?p 0 ?z ?w))"),
        rw!("concatenation-and-pooling-0"     ; "(concat 1 (poolavg ?kx ?ky ?sx ?sy ?p ?x) (poolavg ?kx ?ky ?sx ?sy ?p ?y)) "               => "(poolavg ?kx ?ky ?sx ?sy ?p (concat 1 ?x ?y))"),
        rw!("concatenation-and-pooling-1"     ; "(concat 0 (poolmax ?kx ?ky ?sx ?sy ?p ?x) (poolmax ?kx ?ky ?sx ?sy ?p ?y)) "               => "(poolmax ?kx ?ky ?sx ?sy ?p (concat 0 ?x ?y))"),
        rw!("concatenation-and-pooling-2"     ; "(concat 1 (poolmax ?kx ?ky ?sx ?sy ?p ?x) (poolmax ?kx ?ky ?sx ?sy ?p ?y)) "               => "(poolmax ?kx ?ky ?sx ?sy ?p (concat 1 ?x ?y))"),
        // inverse
        rw!("-ewadd-is-associative"            ;"(ewadd (ewadd ?x ?y) ?z)"                                                => "(ewadd ?x (ewadd ?y ?z)) "                                             ),
        rw!("-ewadd-is-commutative"            ;"(ewadd ?y ?x)"                                                           => "(ewadd ?x ?y) "                                                        ),
        rw!("-ewmul-is-associative"            ;"(ewmul (ewmul ?x ?y) ?z)"                                                => "(ewmul ?x (ewmul ?y ?z)) "                                             ),
        rw!("-ewmul-is-commutative"            ;"(ewmul ?y ?x)"                                                           => "(ewmul ?x ?y) "                                                        ),
        rw!("-distributivity-0"                ;"(ewadd (ewmul ?x ?z) (ewmul ?y ?z))"                                     => "(ewmul (ewadd ?x ?y) ?z) "                                             ),
        rw!("-smul-is-associative"             ;"(smul ?x  (smul ?y ?w))"                                                 => "(smul (smul ?x ?y) ?w) "                                               ),
        rw!("-distributivity-1"                ;"(ewadd (smul ?x ?w)  (smul ?y ?w))"                                      => "(smul (ewadd ?x ?y) ?w) "                                              ),
        rw!("-operator-commutativity-0"        ;"(ewmul ?x  (smul ?y ?w))"                                                => "(smul (ewmul ?x ?y) ?w) "                                              ),
        rw!("-transpose-is-its-own-inverse"    ;"?x"                                                                      => "(transpose (transpose ?x)) "                                           ),
        rw!("-operator-commutativity-1"        ;"(ewadd (transpose ?x)  (transpose ?y))"                                  => "(transpose (ewadd ?x ?y)) "                                            ),
        rw!("-operator-commutativity-2"        ;"(ewmul (transpose ?x)  (transpose ?y))"                                  => "(transpose (ewmul ?x ?y)) "                                            ),
        rw!("-operator-commutativity-3"        ;"(transpose (smul ?x ?w))"                                                => "(smul (transpose ?x) ?w) "                                             ),
        rw!("-matmul-is-associative"           ;"(matmul (matmul ?x ?y) ?z)"                                              => "(matmul ?x (matmul ?y ?z)) "                                           ),
        rw!("-matmul-is-linear-0"              ;"(matmul ?x  (smul ?y ?w))"                                               => "(smul (matmul ?x ?y) ?w) "                                             ),
        rw!("-matmul-is-linear-1"              ;"(ewadd (matmul ?x ?y) (matmul ?x ?z))"                                   => "(matmul ?x (ewadd ?y ?z)) "                                            ),
        rw!("-matmul-and-transpose"            ;"(matmul (transpose ?y)  (transpose ?x))"                                 => "(transpose (matmul ?x ?y)) "                                           ),
        rw!("-conv-is-bilinear-0"              ;"(conv2d ?sx ?sy ?p ?c ?x (smul ?y ?w))"                                  => "(conv2d ?sx ?sy ?p ?c (smul ?x ?w) ?y) "                               ),
        rw!("-conv-is-bilinear-1"              ;"(conv2d ?sx ?sy ?p 0 (smul ?x ?w) ?y)"                               => "(smul (conv2d ?sx ?sy ?p 0 ?x ?y) ?w) "                            ),
        rw!("-conv-is-bilinear-2"              ;"(ewadd (conv2d ?sx ?sy ?p 0 ?x ?y) (conv2d ?sx ?sy ?p 0 ?x ?z))" => "(conv2d ?sx ?sy ?p 0 ?x (ewadd ?y ?z)) "                           ),
        rw!("-conv-is-bilinear-3"              ;"(ewadd (conv2d ?sx ?sy ?p 0 ?x ?z) (conv2d ?sx ?sy ?p 0 ?y ?z))" => "(conv2d ?sx ?sy ?p 0 (ewadd ?x ?y) ?z) "                           ),
        rw!("-enlarge-convolution-kernel"      ;"(conv2d ?sx ?sy 0 ?c ?x (enlarge ?kx ?ky ?y))"                            => "(conv2d ?sx ?sy 0 ?c ?x ?y) "                                      ),
        rw!("-operator-commutativity-4"        ;"(relu (conv2d ?sx ?sy ?p 0 ?x ?y))"                                  => "(conv2d ?sx ?sy ?p 2 ?x ?y) "                                      ),
        rw!("-conv-with-2-applies-relu"    ;"(transpose (relu ?x))"                                                   => "(relu (transpose ?x)) "                                                ),
        rw!("-pooling-by-conv.-with-Cpool"     ;"(poolavg ?kx ?ky ?sx ?sy ?p ?x)"                                                   => "(conv2d ?sx ?sy ?p 0 ?x (Cpool ?kx ?ky)) "                              ),
        rw!("-const_iconv-and-const_pool"      ;"(Cpool ?kx ?ky)"                              => "(poolavg ?kx ?ky 1 1 0 (Iconv ?kx ?ky))"),
        // rw!("-identity-kernel"                 ;"?x"                                                                      => "(conv2d 1 1 0 0 ?x (Iconv ?k)) "                               ),
        rw!("-identity-matrix"                 ;"?x"                                                                      => "(matmul ?x   Imatmul ) "                                               ),
        rw!("-ewmul-identity"                  ;"?x"                                                                      => "(ewmul ?x Iewmul) "                                                    ),
        // rw!("-split-definition-00"              ;"?x"                                                                      => "(split_0 1 (concat 1 ?x ?y)) "                                       ),
        // rw!("-split-definition-01"              ;"?x"                                                                      => "(split_0 0 (concat 0 ?x ?y)) "                                       ),
        // rw!("-split-definition-10"              ;"?y"                                                                      => "(split_1 0 (concat 0 ?x ?y)) "                                       ),
        // rw!("-split-definition-11"              ;"?y"                                                                      => "(split_1 1 (concat 1 ?x ?y)) "                                       ),
        rw!("-geometry-of-concatenation"       ;"(concat 1 (concat 0 ?x ?z) (concat 0 ?y ?w))"                            => "(concat 0 (concat 1 ?x ?y) (concat 1 ?z ?w)) "                         ),
        rw!("-operator-commutativity-5"        ;"(smul (concat ?a ?x ?y) ?w)"                                             => "(concat ?a (smul ?x ?w) (smul ?y ?w)) "                                ),
        rw!("-operator-commutativity-6"        ;"(ewadd (concat ?a ?x ?z) (concat ?a ?y ?w))"                             => "(concat ?a (ewadd ?x ?y) (ewadd ?z ?w)) "                              ),
        rw!("-operator-commutativity-7"        ;"(ewmul (concat ?a ?x ?z) (concat ?a ?y ?w))"                             => "(concat ?a (ewmul ?x ?y) (ewmul ?z ?w)) "                              ),
        rw!("-operator-commutativity-8"        ;"(relu (concat ?a ?x ?y))"                                                => "(concat ?a (relu ?x) (relu ?y)) "                                      ),
        rw!("-concatenation-and-transpose"     ;"(transpose (concat 0 ?x ?y))"                                            => "(concat 1 (transpose ?x) (transpose ?y)) "                             ),
        rw!("-concatenation-and-matrix-mul.-0" ;"(matmul ?x (concat 1 ?y ?z))"                                            => "(concat 1 (matmul ?x ?y) (matmul ?x ?z)) "                             ),
        rw!("-concatenation-and-matrix-mul.-1" ;"(ewadd (matmul ?x ?y) (matmul ?z ?w))"                                   => "(matmul (concat 1 ?x ?z) (concat 0 ?y ?w)) "                           ),
        rw!("-concatenation-and-conv.-0"       ;"(conv2d ?sx ?sy ?p ?c (concat 0 ?x ?y) ?z)"                              => "(concat 0 (conv2d ?sx ?sy ?p ?c ?x ?z) (conv2d ?sx ?sy ?p ?c ?y ?z)) " ),
        rw!("-concatenation-and-conv.-1"       ;"(conv2d ?sx ?sy ?p ?c ?x (concat 0 ?y ?z))"                              => "(concat 1 (conv2d ?sx ?sy ?p ?c ?x ?y) (conv2d ?sx ?sy ?p ?c ?x ?z)) " ),
        rw!("-concatenation-and-conv.-2"       ;"(ewadd (conv2d ?sx ?sy ?p 0 ?x ?y) (conv2d ?sx ?sy ?p 0 ?z ?w))" => "(conv2d ?sx ?sy ?p 0 (concat 1 ?x ?z) (concat 1 ?y ?w)) "          ),
        rw!("-concatenation-and-pooling-0"     ;"(poolavg ?kx ?ky ?sx ?sy ?p (concat 1 ?x ?y))"                                     => "(concat 1 (poolavg ?kx ?ky ?sx ?sy ?p ?x) (poolavg ?kx ?ky ?sx ?sy ?p ?y)) "               ),
        rw!("-concatenation-and-pooling-1"     ;"(poolmax ?kx ?ky ?sx ?sy ?p (concat 0 ?x ?y))"                                     => "(concat 0 (poolmax ?kx ?ky ?sx ?sy ?p ?x) (poolmax ?kx ?ky ?sx ?sy ?p ?y)) "               ),
        rw!("-concatenation-and-pooling-2"     ;"(poolmax ?kx ?ky ?sx ?sy ?p (concat 1 ?x ?y))"                                     => "(concat 1 (poolmax ?kx ?ky ?sx ?sy ?p ?x) (poolmax ?kx ?ky ?sx ?sy ?p ?y)) "               ),
]}
