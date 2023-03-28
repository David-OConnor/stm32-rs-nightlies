#[doc = "Register `APB2FZ1` reader"]
pub struct R(crate::R<APB2FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2FZ1` writer"]
pub struct W(crate::W<APB2FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2FZ1_SPEC>;
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
impl From<crate::W<APB2FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM1` reader - TIM1 stop in debug"]
pub type DBG_TIM1_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM1` writer - TIM1 stop in debug"]
pub type DBG_TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM8` reader - TIM8 stop in debug"]
pub type DBG_TIM8_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM8` writer - TIM8 stop in debug"]
pub type DBG_TIM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM15` reader - TIM15 stop in debug"]
pub type DBG_TIM15_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM15` writer - TIM15 stop in debug"]
pub type DBG_TIM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM16` reader - TIM16 stop in debug"]
pub type DBG_TIM16_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM16` writer - TIM16 stop in debug"]
pub type DBG_TIM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM17` reader - TIM17 stop in debug"]
pub type DBG_TIM17_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM17` writer - TIM17 stop in debug"]
pub type DBG_TIM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_HRTIM` reader - HRTIM stop in debug"]
pub type DBG_HRTIM_R = crate::BitReader<bool>;
#[doc = "Field `DBG_HRTIM` writer - HRTIM stop in debug"]
pub type DBG_HRTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2FZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&self) -> DBG_TIM1_R {
        DBG_TIM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&self) -> DBG_TIM8_R {
        DBG_TIM8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&self) -> DBG_TIM15_R {
        DBG_TIM15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&self) -> DBG_TIM16_R {
        DBG_TIM16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&self) -> DBG_TIM17_R {
        DBG_TIM17_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&self) -> DBG_HRTIM_R {
        DBG_HRTIM_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1(&mut self) -> DBG_TIM1_W<0> {
        DBG_TIM1_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8(&mut self) -> DBG_TIM8_W<1> {
        DBG_TIM8_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15(&mut self) -> DBG_TIM15_W<16> {
        DBG_TIM15_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16(&mut self) -> DBG_TIM16_W<17> {
        DBG_TIM16_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17(&mut self) -> DBG_TIM17_W<18> {
        DBG_TIM17_W::new(self)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_hrtim(&mut self) -> DBG_HRTIM_W<29> {
        DBG_HRTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz1](index.html) module"]
pub struct APB2FZ1_SPEC;
impl crate::RegisterSpec for APB2FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2fz1::R](R) reader structure"]
impl crate::Readable for APB2FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2fz1::W](W) writer structure"]
impl crate::Writable for APB2FZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2FZ1 to value 0"]
impl crate::Resettable for APB2FZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
