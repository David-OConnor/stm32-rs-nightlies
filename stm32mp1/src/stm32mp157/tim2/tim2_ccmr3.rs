#[doc = "Register `TIM2_CCMR3` reader"]
pub struct R(crate::R<TIM2_CCMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_CCMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_CCMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_CCMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_CCMR3` writer"]
pub struct W(crate::W<TIM2_CCMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_CCMR3_SPEC>;
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
impl From<crate::W<TIM2_CCMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_CCMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC5FE` reader - OC5FE"]
pub type OC5FE_R = crate::BitReader<bool>;
#[doc = "Field `OC5FE` writer - OC5FE"]
pub type OC5FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC5PE` reader - OC5PE"]
pub type OC5PE_R = crate::BitReader<bool>;
#[doc = "Field `OC5PE` writer - OC5PE"]
pub type OC5PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC5M` reader - OC5M"]
pub type OC5M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC5M` writer - OC5M"]
pub type OC5M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_CCMR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC5CE` reader - OC5CE"]
pub type OC5CE_R = crate::BitReader<bool>;
#[doc = "Field `OC5CE` writer - OC5CE"]
pub type OC5CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC6FE` reader - OC6FE"]
pub type OC6FE_R = crate::BitReader<bool>;
#[doc = "Field `OC6FE` writer - OC6FE"]
pub type OC6FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC6PE` reader - OC6PE"]
pub type OC6PE_R = crate::BitReader<bool>;
#[doc = "Field `OC6PE` writer - OC6PE"]
pub type OC6PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC6M` reader - OC6M"]
pub type OC6M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC6M` writer - OC6M"]
pub type OC6M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_CCMR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC6CE` reader - OC6CE"]
pub type OC6CE_R = crate::BitReader<bool>;
#[doc = "Field `OC6CE` writer - OC6CE"]
pub type OC6CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC5M3` reader - OC5M3"]
pub type OC5M3_R = crate::BitReader<bool>;
#[doc = "Field `OC5M3` writer - OC5M3"]
pub type OC5M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
#[doc = "Field `OC6M3` reader - OC6M3"]
pub type OC6M3_R = crate::BitReader<bool>;
#[doc = "Field `OC6M3` writer - OC6M3"]
pub type OC6M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM2_CCMR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    pub fn oc5m3(&self) -> OC5M3_R {
        OC5M3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    pub fn oc6m3(&self) -> OC6M3_R {
        OC6M3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OC5FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> OC5FE_W<2> {
        OC5FE_W::new(self)
    }
    #[doc = "Bit 3 - OC5PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> OC5PE_W<3> {
        OC5PE_W::new(self)
    }
    #[doc = "Bits 4:6 - OC5M"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> OC5M_W<4> {
        OC5M_W::new(self)
    }
    #[doc = "Bit 7 - OC5CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> OC5CE_W<7> {
        OC5CE_W::new(self)
    }
    #[doc = "Bit 10 - OC6FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> OC6FE_W<10> {
        OC6FE_W::new(self)
    }
    #[doc = "Bit 11 - OC6PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> OC6PE_W<11> {
        OC6PE_W::new(self)
    }
    #[doc = "Bits 12:14 - OC6M"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> OC6M_W<12> {
        OC6M_W::new(self)
    }
    #[doc = "Bit 15 - OC6CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> OC6CE_W<15> {
        OC6CE_W::new(self)
    }
    #[doc = "Bit 16 - OC5M3"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m3(&mut self) -> OC5M3_W<16> {
        OC5M3_W::new(self)
    }
    #[doc = "Bit 24 - OC6M3"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m3(&mut self) -> OC6M3_W<24> {
        OC6M3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccmr3](index.html) module"]
pub struct TIM2_CCMR3_SPEC;
impl crate::RegisterSpec for TIM2_CCMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim2_ccmr3::R](R) reader structure"]
impl crate::Readable for TIM2_CCMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_ccmr3::W](W) writer structure"]
impl crate::Writable for TIM2_CCMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM2_CCMR3 to value 0"]
impl crate::Resettable for TIM2_CCMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
