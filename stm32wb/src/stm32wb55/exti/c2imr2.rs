#[doc = "Register `C2IMR2` reader"]
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR2` writer"]
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM0_R = crate::BitReader<bool>;
#[doc = "Field `IM0` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM1` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM1_R = crate::BitReader<bool>;
#[doc = "Field `IM1` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM2` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM2_R = crate::BitReader<bool>;
#[doc = "Field `IM2` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM3` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM3_R = crate::BitReader<bool>;
#[doc = "Field `IM3` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM4` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM4_R = crate::BitReader<bool>;
#[doc = "Field `IM4` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM5` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM5_R = crate::BitReader<bool>;
#[doc = "Field `IM5` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM6` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM6_R = crate::BitReader<bool>;
#[doc = "Field `IM6` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM7` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM7_R = crate::BitReader<bool>;
#[doc = "Field `IM7` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM8` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM8_R = crate::BitReader<bool>;
#[doc = "Field `IM8` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM9` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM9_R = crate::BitReader<bool>;
#[doc = "Field `IM9` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM10` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM10_R = crate::BitReader<bool>;
#[doc = "Field `IM10` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM11` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM11_R = crate::BitReader<bool>;
#[doc = "Field `IM11` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM12` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM12_R = crate::BitReader<bool>;
#[doc = "Field `IM12` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM13` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM13_R = crate::BitReader<bool>;
#[doc = "Field `IM13` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM14` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM14_R = crate::BitReader<bool>;
#[doc = "Field `IM14` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM15` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM15_R = crate::BitReader<bool>;
#[doc = "Field `IM15` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `IM16` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM16_R = crate::BitReader<bool>;
#[doc = "Field `IM16` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub type IM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    #[doc = "Bit 1 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    #[doc = "Bit 3 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    #[doc = "Bit 4 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    #[doc = "Bit 5 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    #[doc = "Bit 7 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    #[doc = "Bit 8 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    #[doc = "Bit 9 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im9(&mut self) -> IM9_W<9> {
        IM9_W::new(self)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    #[doc = "Bit 15 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    #[doc = "Bit 16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](index.html) module"]
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr2::R](R) reader structure"]
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](W) writer structure"]
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR2 to value 0x0001_fcfd"]
impl crate::Resettable for C2IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_fcfd;
}
