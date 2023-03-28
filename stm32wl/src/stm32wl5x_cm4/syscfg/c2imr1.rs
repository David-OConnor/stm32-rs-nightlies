#[doc = "Register `C2IMR1` reader"]
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR1` writer"]
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` reader - RTCSTAMPTAMPLSECSSIM"]
pub type RTCSTAMPTAMPLSECSSIM_R = crate::BitReader<bool>;
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` writer - RTCSTAMPTAMPLSECSSIM"]
pub type RTCSTAMPTAMPLSECSSIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RTCALARMIM` reader - RTCALARMIM"]
pub type RTCALARMIM_R = crate::BitReader<bool>;
#[doc = "Field `RTCALARMIM` writer - RTCALARMIM"]
pub type RTCALARMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RTCSSRUIM` reader - RTCSSRUIM"]
pub type RTCSSRUIM_R = crate::BitReader<bool>;
#[doc = "Field `RTCSSRUIM` writer - RTCSSRUIM"]
pub type RTCSSRUIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RTCWKUPIM` reader - RTCWKUPIM"]
pub type RTCWKUPIM_R = crate::BitReader<bool>;
#[doc = "Field `RTCWKUPIM` writer - RTCWKUPIM"]
pub type RTCWKUPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `RCCIM` reader - RCCIM"]
pub type RCCIM_R = crate::BitReader<bool>;
#[doc = "Field `RCCIM` writer - RCCIM"]
pub type RCCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `FLASHIM` reader - FLASHIM"]
pub type FLASHIM_R = crate::BitReader<bool>;
#[doc = "Field `FLASHIM` writer - FLASHIM"]
pub type FLASHIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `PKAIM` reader - PKAIM"]
pub type PKAIM_R = crate::BitReader<bool>;
#[doc = "Field `PKAIM` writer - PKAIM"]
pub type PKAIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `AESIM` reader - AESIM"]
pub type AESIM_R = crate::BitReader<bool>;
#[doc = "Field `AESIM` writer - AESIM"]
pub type AESIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `COMPIM` reader - COMPIM"]
pub type COMPIM_R = crate::BitReader<bool>;
#[doc = "Field `COMPIM` writer - COMPIM"]
pub type COMPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `ADCIM` reader - ADCIM"]
pub type ADCIM_R = crate::BitReader<bool>;
#[doc = "Field `ADCIM` writer - ADCIM"]
pub type ADCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `DACIM` reader - DACIM"]
pub type DACIM_R = crate::BitReader<bool>;
#[doc = "Field `DACIM` writer - DACIM"]
pub type DACIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI0IM` reader - EXTI0IM"]
pub type EXTI0IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI0IM` writer - EXTI0IM"]
pub type EXTI0IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI1IM` reader - EXTI1IM"]
pub type EXTI1IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1IM` writer - EXTI1IM"]
pub type EXTI1IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI2IM` reader - EXTI2IM"]
pub type EXTI2IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI2IM` writer - EXTI2IM"]
pub type EXTI2IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI3IM` reader - EXTI3IM"]
pub type EXTI3IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3IM` writer - EXTI3IM"]
pub type EXTI3IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI4IM` reader - EXTI4IM"]
pub type EXTI4IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI4IM` writer - EXTI4IM"]
pub type EXTI4IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI5IM` reader - EXTI5IM"]
pub type EXTI5IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5IM` writer - EXTI5IM"]
pub type EXTI5IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI6IM` reader - EXTI6IM"]
pub type EXTI6IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6IM` writer - EXTI6IM"]
pub type EXTI6IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI7IM` reader - EXTI7IM"]
pub type EXTI7IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7IM` writer - EXTI7IM"]
pub type EXTI7IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI8IM` reader - EXTI8IM"]
pub type EXTI8IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8IM` writer - EXTI8IM"]
pub type EXTI8IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI9IM` reader - EXTI9IM"]
pub type EXTI9IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9IM` writer - EXTI9IM"]
pub type EXTI9IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI10IM` reader - EXTI10IM"]
pub type EXTI10IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10IM` writer - EXTI10IM"]
pub type EXTI10IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI11IM` reader - EXTI11IM"]
pub type EXTI11IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11IM` writer - EXTI11IM"]
pub type EXTI11IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI12IM` reader - EXTI12IM"]
pub type EXTI12IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12IM` writer - EXTI12IM"]
pub type EXTI12IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI13IM` reader - EXTI13IM"]
pub type EXTI13IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13IM` writer - EXTI13IM"]
pub type EXTI13IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI14IM` reader - EXTI14IM"]
pub type EXTI14IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14IM` writer - EXTI14IM"]
pub type EXTI14IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
#[doc = "Field `EXTI15IM` reader - EXTI15IM"]
pub type EXTI15IM_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15IM` writer - EXTI15IM"]
pub type EXTI15IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&self) -> RTCSTAMPTAMPLSECSSIM_R {
        RTCSTAMPTAMPLSECSSIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTCALARMIM"]
    #[inline(always)]
    pub fn rtcalarmim(&self) -> RTCALARMIM_R {
        RTCALARMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&self) -> RTCSSRUIM_R {
        RTCSSRUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCWKUPIM"]
    #[inline(always)]
    pub fn rtcwkupim(&self) -> RTCWKUPIM_R {
        RTCWKUPIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - RCCIM"]
    #[inline(always)]
    pub fn rccim(&self) -> RCCIM_R {
        RCCIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLASHIM"]
    #[inline(always)]
    pub fn flashim(&self) -> FLASHIM_R {
        FLASHIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - PKAIM"]
    #[inline(always)]
    pub fn pkaim(&self) -> PKAIM_R {
        PKAIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - AESIM"]
    #[inline(always)]
    pub fn aesim(&self) -> AESIM_R {
        AESIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - COMPIM"]
    #[inline(always)]
    pub fn compim(&self) -> COMPIM_R {
        COMPIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADCIM"]
    #[inline(always)]
    pub fn adcim(&self) -> ADCIM_R {
        ADCIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACIM"]
    #[inline(always)]
    pub fn dacim(&self) -> DACIM_R {
        DACIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - EXTI0IM"]
    #[inline(always)]
    pub fn exti0im(&self) -> EXTI0IM_R {
        EXTI0IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EXTI1IM"]
    #[inline(always)]
    pub fn exti1im(&self) -> EXTI1IM_R {
        EXTI1IM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EXTI2IM"]
    #[inline(always)]
    pub fn exti2im(&self) -> EXTI2IM_R {
        EXTI2IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EXTI3IM"]
    #[inline(always)]
    pub fn exti3im(&self) -> EXTI3IM_R {
        EXTI3IM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EXTI4IM"]
    #[inline(always)]
    pub fn exti4im(&self) -> EXTI4IM_R {
        EXTI4IM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    pub fn exti5im(&self) -> EXTI5IM_R {
        EXTI5IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    pub fn exti6im(&self) -> EXTI6IM_R {
        EXTI6IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    pub fn exti7im(&self) -> EXTI7IM_R {
        EXTI7IM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    pub fn exti8im(&self) -> EXTI8IM_R {
        EXTI8IM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    pub fn exti9im(&self) -> EXTI9IM_R {
        EXTI9IM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    pub fn exti10im(&self) -> EXTI10IM_R {
        EXTI10IM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    pub fn exti11im(&self) -> EXTI11IM_R {
        EXTI11IM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    pub fn exti12im(&self) -> EXTI12IM_R {
        EXTI12IM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    pub fn exti13im(&self) -> EXTI13IM_R {
        EXTI13IM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    pub fn exti14im(&self) -> EXTI14IM_R {
        EXTI14IM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    pub fn exti15im(&self) -> EXTI15IM_R {
        EXTI15IM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcstamptamplsecssim(&mut self) -> RTCSTAMPTAMPLSECSSIM_W<0> {
        RTCSTAMPTAMPLSECSSIM_W::new(self)
    }
    #[doc = "Bit 1 - RTCALARMIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcalarmim(&mut self) -> RTCALARMIM_W<1> {
        RTCALARMIM_W::new(self)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcssruim(&mut self) -> RTCSSRUIM_W<2> {
        RTCSSRUIM_W::new(self)
    }
    #[doc = "Bit 3 - RTCWKUPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcwkupim(&mut self) -> RTCWKUPIM_W<3> {
        RTCWKUPIM_W::new(self)
    }
    #[doc = "Bit 5 - RCCIM"]
    #[inline(always)]
    #[must_use]
    pub fn rccim(&mut self) -> RCCIM_W<5> {
        RCCIM_W::new(self)
    }
    #[doc = "Bit 6 - FLASHIM"]
    #[inline(always)]
    #[must_use]
    pub fn flashim(&mut self) -> FLASHIM_W<6> {
        FLASHIM_W::new(self)
    }
    #[doc = "Bit 8 - PKAIM"]
    #[inline(always)]
    #[must_use]
    pub fn pkaim(&mut self) -> PKAIM_W<8> {
        PKAIM_W::new(self)
    }
    #[doc = "Bit 10 - AESIM"]
    #[inline(always)]
    #[must_use]
    pub fn aesim(&mut self) -> AESIM_W<10> {
        AESIM_W::new(self)
    }
    #[doc = "Bit 11 - COMPIM"]
    #[inline(always)]
    #[must_use]
    pub fn compim(&mut self) -> COMPIM_W<11> {
        COMPIM_W::new(self)
    }
    #[doc = "Bit 12 - ADCIM"]
    #[inline(always)]
    #[must_use]
    pub fn adcim(&mut self) -> ADCIM_W<12> {
        ADCIM_W::new(self)
    }
    #[doc = "Bit 13 - DACIM"]
    #[inline(always)]
    #[must_use]
    pub fn dacim(&mut self) -> DACIM_W<13> {
        DACIM_W::new(self)
    }
    #[doc = "Bit 16 - EXTI0IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti0im(&mut self) -> EXTI0IM_W<16> {
        EXTI0IM_W::new(self)
    }
    #[doc = "Bit 17 - EXTI1IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti1im(&mut self) -> EXTI1IM_W<17> {
        EXTI1IM_W::new(self)
    }
    #[doc = "Bit 18 - EXTI2IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti2im(&mut self) -> EXTI2IM_W<18> {
        EXTI2IM_W::new(self)
    }
    #[doc = "Bit 19 - EXTI3IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti3im(&mut self) -> EXTI3IM_W<19> {
        EXTI3IM_W::new(self)
    }
    #[doc = "Bit 20 - EXTI4IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti4im(&mut self) -> EXTI4IM_W<20> {
        EXTI4IM_W::new(self)
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti5im(&mut self) -> EXTI5IM_W<21> {
        EXTI5IM_W::new(self)
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti6im(&mut self) -> EXTI6IM_W<22> {
        EXTI6IM_W::new(self)
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti7im(&mut self) -> EXTI7IM_W<23> {
        EXTI7IM_W::new(self)
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti8im(&mut self) -> EXTI8IM_W<24> {
        EXTI8IM_W::new(self)
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti9im(&mut self) -> EXTI9IM_W<25> {
        EXTI9IM_W::new(self)
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti10im(&mut self) -> EXTI10IM_W<26> {
        EXTI10IM_W::new(self)
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti11im(&mut self) -> EXTI11IM_W<27> {
        EXTI11IM_W::new(self)
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti12im(&mut self) -> EXTI12IM_W<28> {
        EXTI12IM_W::new(self)
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti13im(&mut self) -> EXTI13IM_W<29> {
        EXTI13IM_W::new(self)
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti14im(&mut self) -> EXTI14IM_W<30> {
        EXTI14IM_W::new(self)
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti15im(&mut self) -> EXTI15IM_W<31> {
        EXTI15IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](index.html) module"]
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr1::R](R) reader structure"]
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](W) writer structure"]
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
