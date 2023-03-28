#[doc = "Register `APB1FZR2` reader"]
pub struct R(crate::R<APB1FZR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR2` writer"]
pub struct W(crate::W<APB1FZR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR2_SPEC>;
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
impl From<crate::W<APB1FZR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_I2C4_STOP` reader - I2C4 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C4_STOP_R = crate::BitReader<DBG_I2C4_STOP_A>;
#[doc = "I2C4 SMBUS timeout counter stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C4_STOP_A {
    #[doc = "0: Same behavior as in normal mode"]
    NormalMode = 0,
    #[doc = "1: I2Cx SMBUS timeout is frozen"]
    SmbusTimeoutFrozen = 1,
}
impl From<DBG_I2C4_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C4_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_I2C4_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_I2C4_STOP_A {
        match self.bits {
            false => DBG_I2C4_STOP_A::NormalMode,
            true => DBG_I2C4_STOP_A::SmbusTimeoutFrozen,
        }
    }
    #[doc = "Checks if the value of the field is `NormalMode`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_I2C4_STOP_A::NormalMode
    }
    #[doc = "Checks if the value of the field is `SmbusTimeoutFrozen`"]
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        *self == DBG_I2C4_STOP_A::SmbusTimeoutFrozen
    }
}
#[doc = "Field `DBG_I2C4_STOP` writer - I2C4 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C4_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR2_SPEC, DBG_I2C4_STOP_A, O>;
impl<'a, const O: u8> DBG_I2C4_STOP_W<'a, O> {
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DBG_I2C4_STOP_A::NormalMode)
    }
    #[doc = "I2Cx SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut W {
        self.variant(DBG_I2C4_STOP_A::SmbusTimeoutFrozen)
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP_A>;
#[doc = "LPTIM2 counter stopped when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP_A {
    #[doc = "0: LPTIMx counter clock is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: LPTIMx counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_LPTIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::Continue,
            true => DBG_LPTIM2_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::Stop
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR2_SPEC, DBG_LPTIM2_STOP_A, O>;
impl<'a, const O: u8> DBG_LPTIM2_STOP_W<'a, O> {
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::Continue)
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::Stop)
    }
}
impl R {
    #[doc = "Bit 1 - I2C4 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - I2C4 SMBUS timeout counter stopped when core is halted"]
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Low Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr2](index.html) module"]
pub struct APB1FZR2_SPEC;
impl crate::RegisterSpec for APB1FZR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr2::R](R) reader structure"]
impl crate::Readable for APB1FZR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr2::W](W) writer structure"]
impl crate::Writable for APB1FZR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1FZR2 to value 0"]
impl crate::Resettable for APB1FZR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
