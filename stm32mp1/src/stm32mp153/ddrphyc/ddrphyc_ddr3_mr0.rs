#[doc = "Register `DDRPHYC_DDR3_MR0` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR0` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDRPHYC_DDR3_MR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BL` reader - BL"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - BL"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CL0` reader - CL0"]
pub type CL0_R = crate::BitReader<bool>;
#[doc = "Field `CL0` writer - CL0"]
pub type CL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, bool, O>;
#[doc = "Field `BT` reader - BT"]
pub type BT_R = crate::BitReader<bool>;
#[doc = "Field `BT` writer - BT"]
pub type BT_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, bool, O>;
#[doc = "Field `CL` reader - CL"]
pub type CL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL` writer - CL"]
pub type CL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::BitReader<bool>;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, bool, O>;
#[doc = "Field `DR` reader - DR"]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - DR"]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, bool, O>;
#[doc = "Field `WR` reader - WR"]
pub type WR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR` writer - WR"]
pub type WR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - PD"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, bool, O>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RSVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RSVD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR0_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    pub fn cl0(&self) -> CL0_R {
        CL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BL"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<0> {
        BL_W::new(self)
    }
    #[doc = "Bit 2 - CL0"]
    #[inline(always)]
    #[must_use]
    pub fn cl0(&mut self) -> CL0_W<2> {
        CL0_W::new(self)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    #[must_use]
    pub fn bt(&mut self) -> BT_W<3> {
        BT_W::new(self)
    }
    #[doc = "Bits 4:6 - CL"]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<4> {
        CL_W::new(self)
    }
    #[doc = "Bit 7 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<7> {
        TM_W::new(self)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<8> {
        DR_W::new(self)
    }
    #[doc = "Bits 9:11 - WR"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<9> {
        WR_W::new(self)
    }
    #[doc = "Bit 12 - PD"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<12> {
        PD_W::new(self)
    }
    #[doc = "Bits 13:15 - RSVD"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RSVD_W<13> {
        RSVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR0 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr0](index.html) module"]
pub struct DDRPHYC_DDR3_MR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR0 to value 0x0a52"]
impl crate::Resettable for DDRPHYC_DDR3_MR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a52;
}
