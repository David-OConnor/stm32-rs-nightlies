#[doc = "Register `APB1HFZR` reader"]
pub struct R(crate::R<APB1HFZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HFZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HFZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HFZR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HFZR` writer"]
pub struct W(crate::W<APB1HFZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HFZR_SPEC>;
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
impl From<crate::W<APB1HFZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HFZR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_I2C4_STOP` reader - I2C4 stop in debug"]
pub type DBG_I2C4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C4_STOP` writer - I2C4 stop in debug"]
pub type DBG_I2C4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HFZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HFZR_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1HFZR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - I2C4 stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - I2C4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<1> {
        DBG_I2C4_STOP_W::new(self)
    }
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<5> {
        DBG_LPTIM2_STOP_W::new(self)
    }
    #[doc = "Bit 6 - LPTIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<6> {
        DBG_LPTIM3_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug MCU APB1 freeze register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hfzr](index.html) module"]
pub struct APB1HFZR_SPEC;
impl crate::RegisterSpec for APB1HFZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hfzr::R](R) reader structure"]
impl crate::Readable for APB1HFZR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hfzr::W](W) writer structure"]
impl crate::Writable for APB1HFZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1HFZR to value 0"]
impl crate::Resettable for APB1HFZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
