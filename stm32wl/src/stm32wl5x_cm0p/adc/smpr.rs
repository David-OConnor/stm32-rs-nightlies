#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP1` reader - SMP1"]
pub type SMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP1` writer - SMP1"]
pub type SMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP2` reader - SMP2"]
pub type SMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP2` writer - SMP2"]
pub type SMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMPSEL` reader - SMPSEL"]
pub type SMPSEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SMPSEL` writer - SMPSEL"]
pub type SMPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new((self.bits >> 8) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<0> {
        SMP1_W::new(self)
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<4> {
        SMP2_W::new(self)
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel(&mut self) -> SMPSEL_W<8> {
        SMPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
