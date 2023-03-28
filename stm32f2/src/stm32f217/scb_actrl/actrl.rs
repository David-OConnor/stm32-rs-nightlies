#[doc = "Register `ACTRL` reader"]
pub struct R(crate::R<ACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTRL` writer"]
pub struct W(crate::W<ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRL_SPEC>;
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
impl From<crate::W<ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISFOLD` reader - DISFOLD"]
pub type DISFOLD_R = crate::BitReader<bool>;
#[doc = "Field `DISFOLD` writer - DISFOLD"]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `FPEXCODIS` reader - FPEXCODIS"]
pub type FPEXCODIS_R = crate::BitReader<bool>;
#[doc = "Field `FPEXCODIS` writer - FPEXCODIS"]
pub type FPEXCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISRAMODE` reader - DISRAMODE"]
pub type DISRAMODE_R = crate::BitReader<bool>;
#[doc = "Field `DISRAMODE` writer - DISRAMODE"]
pub type DISRAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISITMATBFLUSH` reader - DISITMATBFLUSH"]
pub type DISITMATBFLUSH_R = crate::BitReader<bool>;
#[doc = "Field `DISITMATBFLUSH` writer - DISITMATBFLUSH"]
pub type DISITMATBFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bit 10 - FPEXCODIS"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<10> {
        FPEXCODIS_W::new(self)
    }
    #[doc = "Bit 11 - DISRAMODE"]
    #[inline(always)]
    #[must_use]
    pub fn disramode(&mut self) -> DISRAMODE_W<11> {
        DISRAMODE_W::new(self)
    }
    #[doc = "Bit 12 - DISITMATBFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<12> {
        DISITMATBFLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrl](index.html) module"]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actrl::R](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actrl::W](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
