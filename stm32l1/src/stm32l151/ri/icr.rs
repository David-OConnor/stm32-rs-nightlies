#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC1IOS` reader - Input capture 1 select bits"]
pub type IC1IOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1IOS` writer - Input capture 1 select bits"]
pub type IC1IOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 4, O>;
#[doc = "Field `IC2IOS` reader - Input capture 2 select bits"]
pub type IC2IOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2IOS` writer - Input capture 2 select bits"]
pub type IC2IOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 4, O>;
#[doc = "Field `IC3IOS` reader - Input capture 3 select bits"]
pub type IC3IOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC3IOS` writer - Input capture 3 select bits"]
pub type IC3IOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 4, O>;
#[doc = "Field `IC4IOS` reader - Input capture 4 select bits"]
pub type IC4IOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC4IOS` writer - Input capture 4 select bits"]
pub type IC4IOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TIM` reader - Timer select bits"]
pub type TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM` writer - Timer select bits"]
pub type TIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IC1` reader - IC1"]
pub type IC1_R = crate::BitReader<bool>;
#[doc = "Field `IC1` writer - IC1"]
pub type IC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `IC2` reader - IC2"]
pub type IC2_R = crate::BitReader<bool>;
#[doc = "Field `IC2` writer - IC2"]
pub type IC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `IC3` reader - IC3"]
pub type IC3_R = crate::BitReader<bool>;
#[doc = "Field `IC3` writer - IC3"]
pub type IC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `IC4` reader - IC4"]
pub type IC4_R = crate::BitReader<bool>;
#[doc = "Field `IC4` writer - IC4"]
pub type IC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&self) -> IC1IOS_R {
        IC1IOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&self) -> IC2IOS_R {
        IC2IOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&self) -> IC3IOS_R {
        IC3IOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&self) -> IC4IOS_R {
        IC4IOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&self) -> IC1_R {
        IC1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&self) -> IC2_R {
        IC2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&self) -> IC3_R {
        IC3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&self) -> IC4_R {
        IC4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic1ios(&mut self) -> IC1IOS_W<0> {
        IC1IOS_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic2ios(&mut self) -> IC2IOS_W<4> {
        IC2IOS_W::new(self)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic3ios(&mut self) -> IC3IOS_W<8> {
        IC3IOS_W::new(self)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic4ios(&mut self) -> IC4IOS_W<12> {
        IC4IOS_W::new(self)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TIM_W<16> {
        TIM_W::new(self)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    #[must_use]
    pub fn ic1(&mut self) -> IC1_W<18> {
        IC1_W::new(self)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    #[must_use]
    pub fn ic2(&mut self) -> IC2_W<19> {
        IC2_W::new(self)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    #[must_use]
    pub fn ic3(&mut self) -> IC3_W<20> {
        IC3_W::new(self)
    }
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    #[must_use]
    pub fn ic4(&mut self) -> IC4_W<21> {
        IC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI input capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
