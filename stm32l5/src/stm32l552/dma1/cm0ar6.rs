#[doc = "Register `CM0AR6` reader"]
pub struct R(crate::R<CM0AR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0AR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0AR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0AR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0AR6` writer"]
pub struct W(crate::W<CM0AR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0AR6_SPEC>;
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
impl From<crate::W<CM0AR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0AR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C1S` reader - DMA channel 1 selection"]
pub type C1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C1S` writer - DMA channel 1 selection"]
pub type C1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C2S` reader - DMA channel 2 selection"]
pub type C2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C2S` writer - DMA channel 2 selection"]
pub type C2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C3S` reader - DMA channel 3 selection"]
pub type C3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C3S` writer - DMA channel 3 selection"]
pub type C3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C4S` reader - DMA channel 4 selection"]
pub type C4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C4S` writer - DMA channel 4 selection"]
pub type C4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C5S` reader - DMA channel 5 selection"]
pub type C5S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C5S` writer - DMA channel 5 selection"]
pub type C5S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C6S` reader - DMA channel 6 selection"]
pub type C6S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C6S` writer - DMA channel 6 selection"]
pub type C6S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `C7S` reader - DMA channel 7 selection"]
pub type C7S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C7S` writer - DMA channel 7 selection"]
pub type C7S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0AR6_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1s(&mut self) -> C1S_W<0> {
        C1S_W::new(self)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c2s(&mut self) -> C2S_W<4> {
        C2S_W::new(self)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c3s(&mut self) -> C3S_W<8> {
        C3S_W::new(self)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c4s(&mut self) -> C4S_W<12> {
        C4S_W::new(self)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c5s(&mut self) -> C5S_W<16> {
        C5S_W::new(self)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c6s(&mut self) -> C6S_W<20> {
        C6S_W::new(self)
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    #[must_use]
    pub fn c7s(&mut self) -> C7S_W<24> {
        C7S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar6](index.html) module"]
pub struct CM0AR6_SPEC;
impl crate::RegisterSpec for CM0AR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0ar6::R](R) reader structure"]
impl crate::Readable for CM0AR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0ar6::W](W) writer structure"]
impl crate::Writable for CM0AR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0AR6 to value 0"]
impl crate::Resettable for CM0AR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
