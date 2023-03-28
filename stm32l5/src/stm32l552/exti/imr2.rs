#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR2` writer"]
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM32` reader - CPU wakeup with interrupt mask on event input"]
pub type IM32_R = crate::BitReader<bool>;
#[doc = "Field `IM32` writer - CPU wakeup with interrupt mask on event input"]
pub type IM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM33` reader - CPU wakeup with interrupt mask on event input"]
pub type IM33_R = crate::BitReader<bool>;
#[doc = "Field `IM33` writer - CPU wakeup with interrupt mask on event input"]
pub type IM33_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM34` reader - CPU wakeup with interrupt mask on event input"]
pub type IM34_R = crate::BitReader<bool>;
#[doc = "Field `IM34` writer - CPU wakeup with interrupt mask on event input"]
pub type IM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM35` reader - CPU wakeup with interrupt mask on event input"]
pub type IM35_R = crate::BitReader<bool>;
#[doc = "Field `IM35` writer - CPU wakeup with interrupt mask on event input"]
pub type IM35_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM36` reader - CPU wakeup with interrupt mask on event input"]
pub type IM36_R = crate::BitReader<bool>;
#[doc = "Field `IM36` writer - CPU wakeup with interrupt mask on event input"]
pub type IM36_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM37` reader - CPU wakeup with interrupt mask on event input"]
pub type IM37_R = crate::BitReader<bool>;
#[doc = "Field `IM37` writer - CPU wakeup with interrupt mask on event input"]
pub type IM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM38` reader - CPU wakeup with interrupt mask on event input"]
pub type IM38_R = crate::BitReader<bool>;
#[doc = "Field `IM38` writer - CPU wakeup with interrupt mask on event input"]
pub type IM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM40` reader - CPU wakeup with interrupt mask on event input"]
pub type IM40_R = crate::BitReader<bool>;
#[doc = "Field `IM40` writer - CPU wakeup with interrupt mask on event input"]
pub type IM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM41` reader - CPU wakeup with interrupt mask on event input"]
pub type IM41_R = crate::BitReader<bool>;
#[doc = "Field `IM41` writer - CPU wakeup with interrupt mask on event input"]
pub type IM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
#[doc = "Field `IM42` reader - CPU wakeup with interrupt mask on event input"]
pub type IM42_R = crate::BitReader<bool>;
#[doc = "Field `IM42` writer - CPU wakeup with interrupt mask on event input"]
pub type IM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<0> {
        IM32_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<1> {
        IM33_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<2> {
        IM34_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> IM35_W<3> {
        IM35_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> IM36_W<4> {
        IM36_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<5> {
        IM37_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<6> {
        IM38_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<8> {
        IM40_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<9> {
        IM41_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr2::W](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0x0787"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0787;
}
