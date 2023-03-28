#[doc = "Register `APB1FZR1` reader"]
pub struct R(crate::R<APB1FZR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR1` writer"]
pub struct W(crate::W<APB1FZR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR1_SPEC>;
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
impl From<crate::W<APB1FZR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_R = crate::BitReader<DBG_TIMER2_STOP_A>;
#[doc = "Debug Timer 2 stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIMER2_STOP_A {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_TIMER2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIMER2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIMER2_STOP_A {
        match self.bits {
            false => DBG_TIMER2_STOP_A::Continue,
            true => DBG_TIMER2_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIMER2_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIMER2_STOP_A::Stop
    }
}
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_TIMER2_STOP_A, O>;
impl<'a, const O: u8> DBG_TIMER2_STOP_W<'a, O> {
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER2_STOP_A::Continue)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER2_STOP_A::Stop)
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_R as DBG_TIM3_STOP_R;
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_R as DBG_TIM4_STOP_R;
#[doc = "Field `DBG_TIM5_STOP` reader - TIM5 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_R as DBG_TIM5_STOP_R;
#[doc = "Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub use DBG_TIMER2_STOP_R as DBG_TIMER6_STOP_R;
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_R as DBG_TIM7_STOP_R;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_W as DBG_TIM3_STOP_W;
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_W as DBG_TIM4_STOP_W;
#[doc = "Field `DBG_TIM5_STOP` writer - TIM5 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_W as DBG_TIM5_STOP_W;
#[doc = "Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub use DBG_TIMER2_STOP_W as DBG_TIMER6_STOP_W;
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted"]
pub use DBG_TIMER2_STOP_W as DBG_TIM7_STOP_W;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP_A>;
#[doc = "Debug RTC stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP_A {
    #[doc = "0: The clock of the RTC counter is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The clock of the RTC counter is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_RTC_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::Continue,
            true => DBG_RTC_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_RTC_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_RTC_STOP_A::Stop
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_RTC_STOP_A, O>;
impl<'a, const O: u8> DBG_RTC_STOP_W<'a, O> {
    #[doc = "The clock of the RTC counter is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::Continue)
    }
    #[doc = "The clock of the RTC counter is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::Stop)
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP_A>;
#[doc = "Debug Window Wachdog stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP_A {
    #[doc = "0: The window watchdog counter clock continues even if the core is halted"]
    Continue = 0,
    #[doc = "1: The window watchdog counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_WWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::Continue,
            true => DBG_WWDG_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_WWDG_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_WWDG_STOP_A::Stop
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_WWDG_STOP_A, O>;
impl<'a, const O: u8> DBG_WWDG_STOP_W<'a, O> {
    #[doc = "The window watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::Continue)
    }
    #[doc = "The window watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::Stop)
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP_A>;
#[doc = "Debug Independent Wachdog stopped when Core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP_A {
    #[doc = "0: The independent watchdog counter clock continues even if the core is halted"]
    Continue = 0,
    #[doc = "1: The independent watchdog counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::Continue,
            true => DBG_IWDG_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_IWDG_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_IWDG_STOP_A::Stop
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_IWDG_STOP_A, O>;
impl<'a, const O: u8> DBG_IWDG_STOP_W<'a, O> {
    #[doc = "The independent watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::Continue)
    }
    #[doc = "The independent watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::Stop)
    }
}
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C1_STOP_R = crate::BitReader<DBG_I2C1_STOP_A>;
#[doc = "I2C1 SMBUS timeout mode stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_STOP_A {
    #[doc = "0: Same behavior as in normal mode"]
    NormalMode = 0,
    #[doc = "1: I2Cx SMBUS timeout is frozen"]
    SmbusTimeoutFrozen = 1,
}
impl From<DBG_I2C1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_I2C1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_I2C1_STOP_A {
        match self.bits {
            false => DBG_I2C1_STOP_A::NormalMode,
            true => DBG_I2C1_STOP_A::SmbusTimeoutFrozen,
        }
    }
    #[doc = "Checks if the value of the field is `NormalMode`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_I2C1_STOP_A::NormalMode
    }
    #[doc = "Checks if the value of the field is `SmbusTimeoutFrozen`"]
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        *self == DBG_I2C1_STOP_A::SmbusTimeoutFrozen
    }
}
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_I2C1_STOP_A, O>;
impl<'a, const O: u8> DBG_I2C1_STOP_W<'a, O> {
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DBG_I2C1_STOP_A::NormalMode)
    }
    #[doc = "I2Cx SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut W {
        self.variant(DBG_I2C1_STOP_A::SmbusTimeoutFrozen)
    }
}
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted"]
pub use DBG_I2C1_STOP_R as DBG_I2C2_STOP_R;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout counter stopped when core is halted"]
pub use DBG_I2C1_STOP_R as DBG_I2C3_STOP_R;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted"]
pub use DBG_I2C1_STOP_W as DBG_I2C2_STOP_W;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout counter stopped when core is halted"]
pub use DBG_I2C1_STOP_W as DBG_I2C3_STOP_W;
#[doc = "Field `DBG_CAN1_STOP` reader - bxCAN stopped when core is halted"]
pub type DBG_CAN1_STOP_R = crate::BitReader<DBG_CAN1_STOP_A>;
#[doc = "bxCAN stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_CAN1_STOP_A {
    #[doc = "0: Same behavior as in normal mode"]
    NormalMode = 0,
    #[doc = "1: The bxCAN1 receive registers are frozen"]
    ReceiveRegistersFrozen = 1,
}
impl From<DBG_CAN1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_CAN1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_CAN1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_CAN1_STOP_A {
        match self.bits {
            false => DBG_CAN1_STOP_A::NormalMode,
            true => DBG_CAN1_STOP_A::ReceiveRegistersFrozen,
        }
    }
    #[doc = "Checks if the value of the field is `NormalMode`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_CAN1_STOP_A::NormalMode
    }
    #[doc = "Checks if the value of the field is `ReceiveRegistersFrozen`"]
    #[inline(always)]
    pub fn is_receive_registers_frozen(&self) -> bool {
        *self == DBG_CAN1_STOP_A::ReceiveRegistersFrozen
    }
}
#[doc = "Field `DBG_CAN1_STOP` writer - bxCAN stopped when core is halted"]
pub type DBG_CAN1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_CAN1_STOP_A, O>;
impl<'a, const O: u8> DBG_CAN1_STOP_W<'a, O> {
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DBG_CAN1_STOP_A::NormalMode)
    }
    #[doc = "The bxCAN1 receive registers are frozen"]
    #[inline(always)]
    pub fn receive_registers_frozen(self) -> &'a mut W {
        self.variant(DBG_CAN1_STOP_A::ReceiveRegistersFrozen)
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader<DBG_LPTIM1_STOP_A>;
#[doc = "LPTIM1 counter stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM1_STOP_A {
    #[doc = "0: LPTIMx counter clock is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: LPTIMx counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_LPTIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_LPTIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM1_STOP_A {
        match self.bits {
            false => DBG_LPTIM1_STOP_A::Continue,
            true => DBG_LPTIM1_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::Stop
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR1_SPEC, DBG_LPTIM1_STOP_A, O>;
impl<'a, const O: u8> DBG_LPTIM1_STOP_W<'a, O> {
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIM1_STOP_A::Continue)
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIM1_STOP_A::Stop)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOP_R {
        DBG_CAN1_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<0> {
        DBG_TIMER2_STOP_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<2> {
        DBG_TIM4_STOP_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<3> {
        DBG_TIM5_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<4> {
        DBG_TIMER6_STOP_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<21> {
        DBG_I2C1_STOP_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<22> {
        DBG_I2C2_STOP_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<23> {
        DBG_I2C3_STOP_W::new(self)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_can1_stop(&mut self) -> DBG_CAN1_STOP_W<25> {
        DBG_CAN1_STOP_W::new(self)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<31> {
        DBG_LPTIM1_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Low Freeze Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr1](index.html) module"]
pub struct APB1FZR1_SPEC;
impl crate::RegisterSpec for APB1FZR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr1::R](R) reader structure"]
impl crate::Readable for APB1FZR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr1::W](W) writer structure"]
impl crate::Writable for APB1FZR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1FZR1 to value 0"]
impl crate::Resettable for APB1FZR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
