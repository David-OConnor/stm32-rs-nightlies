#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_SLEEP` reader - Allow debug in SLEEP mode"]
pub type DBG_SLEEP_R = crate::BitReader<DBG_SLEEP_A>;
#[doc = "Allow debug in SLEEP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_SLEEP_A {
    #[doc = "0: Debug Sleep Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Sleep Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_SLEEP_A {
        match self.bits {
            false => DBG_SLEEP_A::Disabled,
            true => DBG_SLEEP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_SLEEP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_SLEEP_A::Enabled
    }
}
#[doc = "Field `DBG_SLEEP` writer - Allow debug in SLEEP mode"]
pub type DBG_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_SLEEP_A, O>;
impl<'a, const O: u8> DBG_SLEEP_W<'a, O> {
    #[doc = "Debug Sleep Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::Disabled)
    }
    #[doc = "Debug Sleep Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::Enabled)
    }
}
#[doc = "Field `DBG_STOP` reader - Allow debug in STOP mode"]
pub type DBG_STOP_R = crate::BitReader<DBG_STOP_A>;
#[doc = "Allow debug in STOP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP_A {
    #[doc = "0: Debug Stop Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Stop Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::Disabled,
            true => DBG_STOP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STOP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STOP_A::Enabled
    }
}
#[doc = "Field `DBG_STOP` writer - Allow debug in STOP mode"]
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_STOP_A, O>;
impl<'a, const O: u8> DBG_STOP_W<'a, O> {
    #[doc = "Debug Stop Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::Disabled)
    }
    #[doc = "Debug Stop Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::Enabled)
    }
}
#[doc = "Field `DBG_STANDBY` reader - Allow debug in STANDBY mode"]
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY_A>;
#[doc = "Allow debug in STANDBY mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY_A {
    #[doc = "0: Debug Standby Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Standby Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STANDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::Disabled,
            true => DBG_STANDBY_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STANDBY_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STANDBY_A::Enabled
    }
}
#[doc = "Field `DBG_STANDBY` writer - Allow debug in STANDBY mode"]
pub type DBG_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_STANDBY_A, O>;
impl<'a, const O: u8> DBG_STANDBY_W<'a, O> {
    #[doc = "Debug Standby Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::Disabled)
    }
    #[doc = "Debug Standby Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Allow debug in SLEEP mode"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow debug in STOP mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow debug in STANDBY mode"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow debug in SLEEP mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<0> {
        DBG_SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - Allow debug in STOP mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Allow debug in STANDBY mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<2> {
        DBG_STANDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
