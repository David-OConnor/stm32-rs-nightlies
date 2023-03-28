#[doc = "Register `TIMCICR` writer"]
pub struct W(crate::W<TIMCICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCICR_SPEC>;
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
impl From<crate::W<TIMCICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1C` writer - Compare 1 Interrupt flag Clear"]
pub type CMP1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `CMP2C` writer - Compare 2 Interrupt flag Clear"]
pub type CMP2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `CMP3C` writer - Compare 3 Interrupt flag Clear"]
pub type CMP3C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `CMP4C` writer - Compare 4 Interrupt flag Clear"]
pub type CMP4C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `REPC` writer - Repetition Interrupt flag Clear"]
pub type REPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `UPDC` writer - Update Interrupt flag Clear"]
pub type UPDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `CPT1C` writer - Capture1 Interrupt flag Clear"]
pub type CPT1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `CPT2C` writer - Capture2 Interrupt flag Clear"]
pub type CPT2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `SET1xC` writer - Output 1 Set flag Clear"]
pub type SET1X_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `RSTx1C` writer - Output 1 Reset flag Clear"]
pub type RSTX1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `SET2xC` writer - Output 2 Set flag Clear"]
pub type SET2X_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `RSTx2C` writer - Output 2 Reset flag Clear"]
pub type RSTX2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `RSTC` writer - Reset Interrupt flag Clear"]
pub type RSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
#[doc = "Field `DLYPRTC` writer - Delayed Protection Flag Clear"]
pub type DLYPRTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> CMP1C_W<0> {
        CMP1C_W::new(self)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> CMP2C_W<1> {
        CMP2C_W::new(self)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> CMP3C_W<2> {
        CMP3C_W::new(self)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> CMP4C_W<3> {
        CMP4C_W::new(self)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> REPC_W<4> {
        REPC_W::new(self)
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<6> {
        UPDC_W::new(self)
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> CPT1C_W<7> {
        CPT1C_W::new(self)
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> CPT2C_W<8> {
        CPT2C_W::new(self)
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> SET1X_C_W<9> {
        SET1X_C_W::new(self)
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> RSTX1C_W<10> {
        RSTX1C_W::new(self)
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> SET2X_C_W<11> {
        SET2X_C_W::new(self)
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> RSTX2C_W<12> {
        RSTX2C_W::new(self)
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<13> {
        RSTC_W::new(self)
    }
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<14> {
        DLYPRTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcicr](index.html) module"]
pub struct TIMCICR_SPEC;
impl crate::RegisterSpec for TIMCICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [timcicr::W](W) writer structure"]
impl crate::Writable for TIMCICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMCICR to value 0"]
impl crate::Resettable for TIMCICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
