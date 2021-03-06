// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use middle::ty::{Ty, HasTypeFlags};
use rustc::middle::const_eval::ConstVal;
use rustc::mir::repr as mir;
use trans::consts;
use trans::common::{self, Block};

use super::operand::OperandRef;
use super::MirContext;

impl<'bcx, 'tcx> MirContext<'bcx, 'tcx> {
    pub fn trans_constval(&mut self,
                          bcx: Block<'bcx, 'tcx>,
                          cv: &ConstVal,
                          ty: Ty<'tcx>)
                          -> OperandRef<'tcx>
    {
        use super::operand::OperandValue::{Ref, Immediate};

        let ccx = bcx.ccx();
        let val = consts::trans_constval(ccx, cv, ty, bcx.fcx.param_substs);
        let val = if common::type_is_immediate(ccx, ty) {
            Immediate(val)
        } else {
            Ref(val)
        };

        assert!(!ty.has_erasable_regions());

        OperandRef {
            ty: ty,
            val: val
        }
    }

    pub fn trans_constant(&mut self,
                          bcx: Block<'bcx, 'tcx>,
                          constant: &mir::Constant<'tcx>)
                          -> OperandRef<'tcx>
    {
        let constant_ty = bcx.monomorphize(&constant.ty);
        match constant.literal {
            mir::Literal::Item { .. } => {
                unimplemented!()
            }
            mir::Literal::Value { ref value } => {
                self.trans_constval(bcx, value, constant_ty)
            }
        }
    }
}
