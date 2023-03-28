#[doc = "Register `APB1LFZ1` reader"]
pub struct R(crate::R<APB1LFZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LFZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LFZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LFZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LFZ1` writer"]
pub struct W(crate::W<APB1LFZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LFZ1_SPEC>;
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
impl From<crate::W<APB1LFZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LFZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2` reader - TIM2 stop in debug"]
pub type TIM2_R = crate::BitReader<bool>;
#[doc = "Field `TIM2` writer - TIM2 stop in debug"]
pub type TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM3` reader - TIM3 stop in debug"]
pub type TIM3_R = crate::BitReader<bool>;
#[doc = "Field `TIM3` writer - TIM3 stop in debug"]
pub type TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM4` reader - TIM4 stop in debug"]
pub type TIM4_R = crate::BitReader<bool>;
#[doc = "Field `TIM4` writer - TIM4 stop in debug"]
pub type TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM5` reader - TIM5 stop in debug"]
pub type TIM5_R = crate::BitReader<bool>;
#[doc = "Field `TIM5` writer - TIM5 stop in debug"]
pub type TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM6` reader - TIM6 stop in debug"]
pub type TIM6_R = crate::BitReader<bool>;
#[doc = "Field `TIM6` writer - TIM6 stop in debug"]
pub type TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM7` reader - TIM7 stop in debug"]
pub type TIM7_R = crate::BitReader<bool>;
#[doc = "Field `TIM7` writer - TIM7 stop in debug"]
pub type TIM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM12` reader - TIM12 stop in debug"]
pub type TIM12_R = crate::BitReader<bool>;
#[doc = "Field `TIM12` writer - TIM12 stop in debug"]
pub type TIM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM13` reader - TIM13 stop in debug"]
pub type TIM13_R = crate::BitReader<bool>;
#[doc = "Field `TIM13` writer - TIM13 stop in debug"]
pub type TIM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `TIM14` reader - TIM14 stop in debug"]
pub type TIM14_R = crate::BitReader<bool>;
#[doc = "Field `TIM14` writer - TIM14 stop in debug"]
pub type TIM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `LPTIM1` reader - LPTIM1 stop in debug"]
pub type LPTIM1_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1` writer - LPTIM1 stop in debug"]
pub type LPTIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub type I2C2_R = crate::BitReader<bool>;
#[doc = "Field `I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub type I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub type I2C3_R = crate::BitReader<bool>;
#[doc = "Field `I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub type I2C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn tim12(&self) -> TIM12_R {
        TIM12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn tim13(&self) -> TIM13_R {
        TIM13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim2(&mut self) -> TIM2_W<0> {
        TIM2_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim3(&mut self) -> TIM3_W<1> {
        TIM3_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim4(&mut self) -> TIM4_W<2> {
        TIM4_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim5(&mut self) -> TIM5_W<3> {
        TIM5_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim6(&mut self) -> TIM6_W<4> {
        TIM6_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim7(&mut self) -> TIM7_W<5> {
        TIM7_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim12(&mut self) -> TIM12_W<6> {
        TIM12_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim13(&mut self) -> TIM13_W<7> {
        TIM13_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn tim14(&mut self) -> TIM14_W<8> {
        TIM14_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1(&mut self) -> LPTIM1_W<9> {
        LPTIM1_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<21> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<22> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<23> {
        I2C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz1](index.html) module"]
pub struct APB1LFZ1_SPEC;
impl crate::RegisterSpec for APB1LFZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lfz1::R](R) reader structure"]
impl crate::Readable for APB1LFZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lfz1::W](W) writer structure"]
impl crate::Writable for APB1LFZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LFZ1 to value 0"]
impl crate::Resettable for APB1LFZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
