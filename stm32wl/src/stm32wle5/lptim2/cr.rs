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
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: LPTIM is disabled"]
    Disabled = 0,
    #[doc = "1: LPTIM is enabled"]
    Enabled = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::Disabled,
            true => ENABLE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::Enabled
    }
}
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "LPTIM is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Disabled)
    }
    #[doc = "LPTIM is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Enabled)
    }
}
#[doc = "Field `SNGSTRT` reader - SNGSTRT"]
pub type SNGSTRT_R = crate::BitReader<SNGSTRTW_A>;
#[doc = "SNGSTRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNGSTRTW_A {
    #[doc = "1: LPTIM start in Single mode"]
    Start = 1,
}
impl From<SNGSTRTW_A> for bool {
    #[inline(always)]
    fn from(variant: SNGSTRTW_A) -> Self {
        variant as u8 != 0
    }
}
impl SNGSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SNGSTRTW_A> {
        match self.bits {
            true => Some(SNGSTRTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SNGSTRTW_A::Start
    }
}
#[doc = "Field `SNGSTRT` writer - SNGSTRT"]
pub type SNGSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SNGSTRTW_A, O>;
impl<'a, const O: u8> SNGSTRT_W<'a, O> {
    #[doc = "LPTIM start in Single mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SNGSTRTW_A::Start)
    }
}
#[doc = "Field `CNTSTRT` reader - CNTSTRT"]
pub type CNTSTRT_R = crate::BitReader<CNTSTRTW_A>;
#[doc = "CNTSTRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTSTRTW_A {
    #[doc = "1: Timer start in Continuous mode"]
    Start = 1,
}
impl From<CNTSTRTW_A> for bool {
    #[inline(always)]
    fn from(variant: CNTSTRTW_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTSTRTW_A> {
        match self.bits {
            true => Some(CNTSTRTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CNTSTRTW_A::Start
    }
}
#[doc = "Field `CNTSTRT` writer - CNTSTRT"]
pub type CNTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CNTSTRTW_A, O>;
impl<'a, const O: u8> CNTSTRT_W<'a, O> {
    #[doc = "Timer start in Continuous mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CNTSTRTW_A::Start)
    }
}
#[doc = "Field `COUNTRST` reader - COUNTRST"]
pub type COUNTRST_R = crate::BitReader<COUNTRSTR_A>;
#[doc = "COUNTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTR_A {
    #[doc = "0: Triggering of reset is possible"]
    Idle = 0,
    #[doc = "1: Reset in progress, do not write 1 to this field"]
    Busy = 1,
}
impl From<COUNTRSTR_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl COUNTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNTRSTR_A {
        match self.bits {
            false => COUNTRSTR_A::Idle,
            true => COUNTRSTR_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == COUNTRSTR_A::Idle
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == COUNTRSTR_A::Busy
    }
}
#[doc = "COUNTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTW_AW {
    #[doc = "1: Trigger synchronous reset of CNT (3 LPTimer core clock cycles)"]
    Reset = 1,
}
impl From<COUNTRSTW_AW> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTRST` writer - COUNTRST"]
pub type COUNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, COUNTRSTW_AW, O>;
impl<'a, const O: u8> COUNTRST_W<'a, O> {
    #[doc = "Trigger synchronous reset of CNT (3 LPTimer core clock cycles)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(COUNTRSTW_AW::Reset)
    }
}
#[doc = "Field `RSTARE` reader - RSTARE"]
pub type RSTARE_R = crate::BitReader<RSTARE_A>;
#[doc = "RSTARE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTARE_A {
    #[doc = "0: CNT Register reads do not trigger reset"]
    Disabled = 0,
    #[doc = "1: CNT Register reads trigger reset of LPTIM"]
    Enabled = 1,
}
impl From<RSTARE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTARE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTARE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTARE_A {
        match self.bits {
            false => RSTARE_A::Disabled,
            true => RSTARE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTARE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTARE_A::Enabled
    }
}
#[doc = "Field `RSTARE` writer - RSTARE"]
pub type RSTARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RSTARE_A, O>;
impl<'a, const O: u8> RSTARE_W<'a, O> {
    #[doc = "CNT Register reads do not trigger reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTARE_A::Disabled)
    }
    #[doc = "CNT Register reads trigger reset of LPTIM"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTARE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<1> {
        SNGSTRT_W::new(self)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<2> {
        CNTSTRT_W::new(self)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<3> {
        COUNTRST_W::new(self)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<4> {
        RSTARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
