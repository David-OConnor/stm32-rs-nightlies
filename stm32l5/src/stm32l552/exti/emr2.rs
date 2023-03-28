#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM32` reader - CPU wakeup with interrupt mask on event input"]
pub type EM32_R = crate::BitReader<bool>;
#[doc = "Field `EM32` writer - CPU wakeup with interrupt mask on event input"]
pub type EM32_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM33` reader - CPU wakeup with interrupt mask on event input"]
pub type EM33_R = crate::BitReader<bool>;
#[doc = "Field `EM33` writer - CPU wakeup with interrupt mask on event input"]
pub type EM33_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM34` reader - CPU wakeup with interrupt mask on event input"]
pub type EM34_R = crate::BitReader<bool>;
#[doc = "Field `EM34` writer - CPU wakeup with interrupt mask on event input"]
pub type EM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM35` reader - CPU wakeup with interrupt mask on event input"]
pub type EM35_R = crate::BitReader<bool>;
#[doc = "Field `EM35` writer - CPU wakeup with interrupt mask on event input"]
pub type EM35_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM36` reader - CPU wakeup with interrupt mask on event input"]
pub type EM36_R = crate::BitReader<bool>;
#[doc = "Field `EM36` writer - CPU wakeup with interrupt mask on event input"]
pub type EM36_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM37` reader - CPU wakeup with interrupt mask on event input"]
pub type EM37_R = crate::BitReader<bool>;
#[doc = "Field `EM37` writer - CPU wakeup with interrupt mask on event input"]
pub type EM37_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM38` reader - CPU wakeup with interrupt mask on event input"]
pub type EM38_R = crate::BitReader<bool>;
#[doc = "Field `EM38` writer - CPU wakeup with interrupt mask on event input"]
pub type EM38_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM40` reader - CPU wakeup with interrupt mask on event input"]
pub type EM40_R = crate::BitReader<bool>;
#[doc = "Field `EM40` writer - CPU wakeup with interrupt mask on event input"]
pub type EM40_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM41` reader - CPU wakeup with interrupt mask on event input"]
pub type EM41_R = crate::BitReader<bool>;
#[doc = "Field `EM41` writer - CPU wakeup with interrupt mask on event input"]
pub type EM41_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
#[doc = "Field `EM42` reader - CPU wakeup with interrupt mask on event input"]
pub type EM42_R = crate::BitReader<bool>;
#[doc = "Field `EM42` writer - CPU wakeup with interrupt mask on event input"]
pub type EM42_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> EM32_W<0> {
        EM32_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> EM33_W<1> {
        EM33_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> EM34_W<2> {
        EM34_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> EM35_W<3> {
        EM35_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em36(&mut self) -> EM36_W<4> {
        EM36_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> EM37_W<5> {
        EM37_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em38(&mut self) -> EM38_W<6> {
        EM38_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<8> {
        EM40_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<9> {
        EM41_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em42(&mut self) -> EM42_W<10> {
        EM42_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
