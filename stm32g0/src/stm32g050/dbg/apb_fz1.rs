#[doc = "Register `APB_FZ1` reader"]
pub struct R(crate::R<APB_FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_FZ1` writer"]
pub struct W(crate::W<APB_FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ1_SPEC>;
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
impl From<crate::W<APB_FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM6_STOP` reader - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM6_STOP` writer - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM7_STOP` reader - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM7_STOP` writer - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted"]
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB_FZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<4> {
        DBG_TIM6_STOP_W::new(self)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<21> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 22 - SMBUS timeout when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<22> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG APB freeze register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_fz1](index.html) module"]
pub struct APB_FZ1_SPEC;
impl crate::RegisterSpec for APB_FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_fz1::R](R) reader structure"]
impl crate::Readable for APB_FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_fz1::W](W) writer structure"]
impl crate::Writable for APB_FZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_FZ1 to value 0"]
impl crate::Resettable for APB_FZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
