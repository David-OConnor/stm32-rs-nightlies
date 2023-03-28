#[doc = "Register `APB3FZR` reader"]
pub struct R(crate::R<APB3FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3FZR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3FZR` writer"]
pub struct W(crate::W<APB3FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3FZR_SPEC>;
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
impl From<crate::W<APB3FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3FZR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_I2C4_STOP` reader - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C4_STOP` writer - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM4_STOP` reader - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM4_STOP` writer - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM5_STOP` reader - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM5_STOP` writer - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM6_STOP` reader - LPTIM6 stop in debug"]
pub type DBG_LPTIM6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM6_STOP` writer - LPTIM6 stop in debug"]
pub type DBG_LPTIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in debug"]
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in debug"]
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3FZR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4_stop(&self) -> DBG_LPTIM4_STOP_R {
        DBG_LPTIM4_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5_stop(&self) -> DBG_LPTIM5_STOP_R {
        DBG_LPTIM5_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPTIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim6_stop(&self) -> DBG_LPTIM6_STOP_R {
        DBG_LPTIM6_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<10> {
        DBG_I2C3_STOP_W::new(self)
    }
    #[doc = "Bit 11 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<11> {
        DBG_I2C4_STOP_W::new(self)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<17> {
        DBG_LPTIM1_STOP_W::new(self)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<18> {
        DBG_LPTIM3_STOP_W::new(self)
    }
    #[doc = "Bit 19 - LPTIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim4_stop(&mut self) -> DBG_LPTIM4_STOP_W<19> {
        DBG_LPTIM4_STOP_W::new(self)
    }
    #[doc = "Bit 20 - LPTIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim5_stop(&mut self) -> DBG_LPTIM5_STOP_W<20> {
        DBG_LPTIM5_STOP_W::new(self)
    }
    #[doc = "Bit 21 - LPTIM6 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim6_stop(&mut self) -> DBG_LPTIM6_STOP_W<21> {
        DBG_LPTIM6_STOP_W::new(self)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<30> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3fzr](index.html) module"]
pub struct APB3FZR_SPEC;
impl crate::RegisterSpec for APB3FZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3fzr::R](R) reader structure"]
impl crate::Readable for APB3FZR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3fzr::W](W) writer structure"]
impl crate::Writable for APB3FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3FZR to value 0"]
impl crate::Resettable for APB3FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
