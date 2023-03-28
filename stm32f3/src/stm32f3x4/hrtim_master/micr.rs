#[doc = "Register `MICR` writer"]
pub struct W(crate::W<MICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MICR_SPEC>;
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
impl From<crate::W<MICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Compare 1 Interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1C_AW {
    #[doc = "1: Clears flag in MISR register"]
    Clear = 1,
}
impl From<MCMP1C_AW> for bool {
    #[inline(always)]
    fn from(variant: MCMP1C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCMP1C` writer - Master Compare 1 Interrupt flag clear"]
pub type MCMP1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, MICR_SPEC, MCMP1C_AW, O>;
impl<'a, const O: u8> MCMP1C_W<'a, O> {
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MCMP1C_AW::Clear)
    }
}
#[doc = "Field `MCMP2C` writer - Master Compare 2 Interrupt flag clear"]
pub use MCMP1C_W as MCMP2C_W;
#[doc = "Field `MCMP3C` writer - Master Compare 3 Interrupt flag clear"]
pub use MCMP1C_W as MCMP3C_W;
#[doc = "Field `MCMP4C` writer - Master Compare 4 Interrupt flag clear"]
pub use MCMP1C_W as MCMP4C_W;
#[doc = "Field `MREPC` writer - Repetition Interrupt flag clear"]
pub use MCMP1C_W as MREPC_W;
#[doc = "Field `SYNCC` writer - Sync Input Interrupt flag clear"]
pub use MCMP1C_W as SYNCC_W;
#[doc = "Field `MUPDC` writer - Master update Interrupt flag clear"]
pub use MCMP1C_W as MUPDC_W;
impl W {
    #[doc = "Bit 0 - Master Compare 1 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1c(&mut self) -> MCMP1C_W<0> {
        MCMP1C_W::new(self)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2c(&mut self) -> MCMP2C_W<1> {
        MCMP2C_W::new(self)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3c(&mut self) -> MCMP3C_W<2> {
        MCMP3C_W::new(self)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4c(&mut self) -> MCMP4C_W<3> {
        MCMP4C_W::new(self)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mrepc(&mut self) -> MREPC_W<4> {
        MREPC_W::new(self)
    }
    #[doc = "Bit 5 - Sync Input Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn syncc(&mut self) -> SYNCC_W<5> {
        SYNCC_W::new(self)
    }
    #[doc = "Bit 6 - Master update Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mupdc(&mut self) -> MUPDC_W<6> {
        MUPDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](index.html) module"]
pub struct MICR_SPEC;
impl crate::RegisterSpec for MICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [micr::W](W) writer structure"]
impl crate::Writable for MICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
