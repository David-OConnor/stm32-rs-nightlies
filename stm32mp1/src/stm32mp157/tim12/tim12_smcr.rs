#[doc = "Register `TIM12_SMCR` reader"]
pub struct R(crate::R<TIM12_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM12_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM12_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM12_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM12_SMCR` writer"]
pub struct W(crate::W<TIM12_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM12_SMCR_SPEC>;
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
impl From<crate::W<TIM12_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM12_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMS` reader - SMS"]
pub type SMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMS` writer - SMS"]
pub type SMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM12_SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TS` reader - TS"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` writer - TS"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM12_SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSM` reader - MSM"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - MSM"]
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM12_SMCR_SPEC, bool, O>;
#[doc = "Field `SMS_3` reader - SMS_3"]
pub type SMS_3_R = crate::BitReader<bool>;
#[doc = "Field `SMS_3` writer - SMS_3"]
pub type SMS_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM12_SMCR_SPEC, bool, O>;
#[doc = "Field `TS_3` reader - TS_3"]
pub type TS_3_R = crate::BitReader<bool>;
#[doc = "Field `TS_3` writer - TS_3"]
pub type TS_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM12_SMCR_SPEC, bool, O>;
#[doc = "Field `TS_4` reader - TS_4"]
pub type TS_4_R = crate::BitReader<bool>;
#[doc = "Field `TS_4` writer - TS_4"]
pub type TS_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM12_SMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    pub fn ts_3(&self) -> TS_3_R {
        TS_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    pub fn ts_4(&self) -> TS_4_R {
        TS_4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> SMS_3_W<16> {
        SMS_3_W::new(self)
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    #[must_use]
    pub fn ts_3(&mut self) -> TS_3_W<20> {
        TS_3_W::new(self)
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    #[must_use]
    pub fn ts_4(&mut self) -> TS_4_W<21> {
        TS_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM12 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_smcr](index.html) module"]
pub struct TIM12_SMCR_SPEC;
impl crate::RegisterSpec for TIM12_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim12_smcr::R](R) reader structure"]
impl crate::Readable for TIM12_SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim12_smcr::W](W) writer structure"]
impl crate::Writable for TIM12_SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM12_SMCR to value 0"]
impl crate::Resettable for TIM12_SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
