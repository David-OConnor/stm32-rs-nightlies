#[doc = "Register `CR5` reader"]
pub struct R(crate::R<CR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR5` writer"]
pub struct W(crate::W<CR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR5_SPEC>;
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
impl From<crate::W<CR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFEOLEN` reader - Enable Radio End Of Life detector enabled"]
pub type RFEOLEN_R = crate::BitReader<RFEOLEN_A>;
#[doc = "Enable Radio End Of Life detector enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLEN_A {
    #[doc = "0: Radio end-of-life detector disabled"]
    Disabled = 0,
    #[doc = "1: Radio end-of-life detector enabled"]
    Enabled = 1,
}
impl From<RFEOLEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEOLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEOLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEOLEN_A {
        match self.bits {
            false => RFEOLEN_A::Disabled,
            true => RFEOLEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFEOLEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFEOLEN_A::Enabled
    }
}
#[doc = "Field `RFEOLEN` writer - Enable Radio End Of Life detector enabled"]
pub type RFEOLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, RFEOLEN_A, O>;
impl<'a, const O: u8> RFEOLEN_W<'a, O> {
    #[doc = "Radio end-of-life detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFEOLEN_A::Disabled)
    }
    #[doc = "Radio end-of-life detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFEOLEN_A::Enabled)
    }
}
#[doc = "Field `SMPSEN` reader - Enable SMPS Step Down converter SMPS mode enabled."]
pub type SMPSEN_R = crate::BitReader<SMPSEN_A>;
#[doc = "Enable SMPS Step Down converter SMPS mode enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEN_A {
    #[doc = "0: SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    Disabled = 0,
    #[doc = "1: SMPS step-down converter SMPS mode enabled"]
    Enabled = 1,
}
impl From<SMPSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSEN_A {
        match self.bits {
            false => SMPSEN_A::Disabled,
            true => SMPSEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPSEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPSEN_A::Enabled
    }
}
#[doc = "Field `SMPSEN` writer - Enable SMPS Step Down converter SMPS mode enabled."]
pub type SMPSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR5_SPEC, SMPSEN_A, O>;
impl<'a, const O: u8> SMPSEN_W<'a, O> {
    #[doc = "SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMPSEN_A::Disabled)
    }
    #[doc = "SMPS step-down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMPSEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    pub fn rfeolen(&self) -> RFEOLEN_R {
        RFEOLEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Enable Radio End Of Life detector enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rfeolen(&mut self) -> RFEOLEN_W<14> {
        RFEOLEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable SMPS Step Down converter SMPS mode enabled."]
    #[inline(always)]
    #[must_use]
    pub fn smpsen(&mut self) -> SMPSEN_W<15> {
        SMPSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](index.html) module"]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr5::R](R) reader structure"]
impl crate::Readable for CR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr5::W](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR5 to value 0"]
impl crate::Resettable for CR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
