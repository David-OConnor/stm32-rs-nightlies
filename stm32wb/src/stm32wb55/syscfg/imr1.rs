#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1IM` reader - Peripheral TIM1 interrupt mask to CPU1"]
pub type TIM1IM_R = crate::BitReader<bool>;
#[doc = "Field `TIM1IM` writer - Peripheral TIM1 interrupt mask to CPU1"]
pub type TIM1IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `TIM16IM` reader - Peripheral TIM16 interrupt mask to CPU1"]
pub type TIM16IM_R = crate::BitReader<bool>;
#[doc = "Field `TIM16IM` writer - Peripheral TIM16 interrupt mask to CPU1"]
pub type TIM16IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `TIM17IM` reader - Peripheral TIM17 interrupt mask to CPU1"]
pub type TIM17IM_R = crate::BitReader<bool>;
#[doc = "Field `TIM17IM` writer - Peripheral TIM17 interrupt mask to CPU1"]
pub type TIM17IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT5IM` reader - Peripheral EXIT5 interrupt mask to CPU1"]
pub type EXIT5IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT5IM` writer - Peripheral EXIT5 interrupt mask to CPU1"]
pub type EXIT5IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT6IM` reader - Peripheral EXIT6 interrupt mask to CPU1"]
pub type EXIT6IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT6IM` writer - Peripheral EXIT6 interrupt mask to CPU1"]
pub type EXIT6IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT7IM` reader - Peripheral EXIT7 interrupt mask to CPU1"]
pub type EXIT7IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT7IM` writer - Peripheral EXIT7 interrupt mask to CPU1"]
pub type EXIT7IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT8IM` reader - Peripheral EXIT8 interrupt mask to CPU1"]
pub type EXIT8IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT8IM` writer - Peripheral EXIT8 interrupt mask to CPU1"]
pub type EXIT8IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT9IM` reader - Peripheral EXIT9 interrupt mask to CPU1"]
pub type EXIT9IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT9IM` writer - Peripheral EXIT9 interrupt mask to CPU1"]
pub type EXIT9IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT10IM` reader - Peripheral EXIT10 interrupt mask to CPU1"]
pub type EXIT10IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT10IM` writer - Peripheral EXIT10 interrupt mask to CPU1"]
pub type EXIT10IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT11IM` reader - Peripheral EXIT11 interrupt mask to CPU1"]
pub type EXIT11IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT11IM` writer - Peripheral EXIT11 interrupt mask to CPU1"]
pub type EXIT11IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT12IM` reader - Peripheral EXIT12 interrupt mask to CPU1"]
pub type EXIT12IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT12IM` writer - Peripheral EXIT12 interrupt mask to CPU1"]
pub type EXIT12IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT13IM` reader - Peripheral EXIT13 interrupt mask to CPU1"]
pub type EXIT13IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT13IM` writer - Peripheral EXIT13 interrupt mask to CPU1"]
pub type EXIT13IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT14IM` reader - Peripheral EXIT14 interrupt mask to CPU1"]
pub type EXIT14IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT14IM` writer - Peripheral EXIT14 interrupt mask to CPU1"]
pub type EXIT14IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
#[doc = "Field `EXIT15IM` reader - Peripheral EXIT15 interrupt mask to CPU1"]
pub type EXIT15IM_R = crate::BitReader<bool>;
#[doc = "Field `EXIT15IM` writer - Peripheral EXIT15 interrupt mask to CPU1"]
pub type EXIT15IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim1im(&self) -> TIM1IM_R {
        TIM1IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim16im(&self) -> TIM16IM_R {
        TIM16IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tim17im(&self) -> TIM17IM_R {
        TIM17IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit5im(&self) -> EXIT5IM_R {
        EXIT5IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit6im(&self) -> EXIT6IM_R {
        EXIT6IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit7im(&self) -> EXIT7IM_R {
        EXIT7IM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit8im(&self) -> EXIT8IM_R {
        EXIT8IM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit9im(&self) -> EXIT9IM_R {
        EXIT9IM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit10im(&self) -> EXIT10IM_R {
        EXIT10IM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit11im(&self) -> EXIT11IM_R {
        EXIT11IM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit12im(&self) -> EXIT12IM_R {
        EXIT12IM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit13im(&self) -> EXIT13IM_R {
        EXIT13IM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit14im(&self) -> EXIT14IM_R {
        EXIT14IM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn exit15im(&self) -> EXIT15IM_R {
        EXIT15IM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Peripheral TIM1 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1im(&mut self) -> TIM1IM_W<13> {
        TIM1IM_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral TIM16 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn tim16im(&mut self) -> TIM16IM_W<14> {
        TIM16IM_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral TIM17 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn tim17im(&mut self) -> TIM17IM_W<15> {
        TIM17IM_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral EXIT5 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit5im(&mut self) -> EXIT5IM_W<21> {
        EXIT5IM_W::new(self)
    }
    #[doc = "Bit 22 - Peripheral EXIT6 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit6im(&mut self) -> EXIT6IM_W<22> {
        EXIT6IM_W::new(self)
    }
    #[doc = "Bit 23 - Peripheral EXIT7 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit7im(&mut self) -> EXIT7IM_W<23> {
        EXIT7IM_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral EXIT8 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit8im(&mut self) -> EXIT8IM_W<24> {
        EXIT8IM_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral EXIT9 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit9im(&mut self) -> EXIT9IM_W<25> {
        EXIT9IM_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral EXIT10 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit10im(&mut self) -> EXIT10IM_W<26> {
        EXIT10IM_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral EXIT11 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit11im(&mut self) -> EXIT11IM_W<27> {
        EXIT11IM_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral EXIT12 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit12im(&mut self) -> EXIT12IM_W<28> {
        EXIT12IM_W::new(self)
    }
    #[doc = "Bit 29 - Peripheral EXIT13 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit13im(&mut self) -> EXIT13IM_W<29> {
        EXIT13IM_W::new(self)
    }
    #[doc = "Bit 30 - Peripheral EXIT14 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit14im(&mut self) -> EXIT14IM_W<30> {
        EXIT14IM_W::new(self)
    }
    #[doc = "Bit 31 - Peripheral EXIT15 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn exit15im(&mut self) -> EXIT15IM_W<31> {
        EXIT15IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU1 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
