#[doc = "Register `SFR` reader"]
pub struct R(crate::R<SFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFSA` reader - Secure Flash start address"]
pub type SFSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFSA` writer - Secure Flash start address"]
pub type SFSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFR_SPEC, u8, u8, 7, O>;
#[doc = "Field `FSD` reader - Flash security disabled"]
pub type FSD_R = crate::BitReader<FSD_A>;
#[doc = "Flash security disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSD_A {
    #[doc = "0: System and Flash memory secure"]
    Secure = 0,
    #[doc = "1: System and Flash memory non-secure"]
    NonSecure = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
impl FSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::Secure,
            true => FSD_A::NonSecure,
        }
    }
    #[doc = "Checks if the value of the field is `Secure`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == FSD_A::Secure
    }
    #[doc = "Checks if the value of the field is `NonSecure`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == FSD_A::NonSecure
    }
}
#[doc = "Field `FSD` writer - Flash security disabled"]
pub type FSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, FSD_A, O>;
impl<'a, const O: u8> FSD_W<'a, O> {
    #[doc = "System and Flash memory secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(FSD_A::Secure)
    }
    #[doc = "System and Flash memory non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(FSD_A::NonSecure)
    }
}
#[doc = "Field `DDS` reader - DDS"]
pub type DDS_R = crate::BitReader<DDS_A>;
#[doc = "DDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS_A {
    #[doc = "0: CPU2 debug access enabled"]
    Enabled = 0,
    #[doc = "1: CPU2 debug access disabled"]
    Disabled = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
impl DDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::Enabled,
            true => DDS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDS_A::Disabled
    }
}
#[doc = "Field `DDS` writer - DDS"]
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, DDS_A, O>;
impl<'a, const O: u8> DDS_W<'a, O> {
    #[doc = "CPU2 debug access enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDS_A::Enabled)
    }
    #[doc = "CPU2 debug access disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDS_A::Disabled)
    }
}
#[doc = "Field `HDPSA` reader - User Flash hide protection area start address"]
pub type HDPSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDPSA` writer - User Flash hide protection area start address"]
pub type HDPSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFR_SPEC, u8, u8, 7, O>;
#[doc = "Field `HDPAD` reader - User Flash hide protection area disabled"]
pub type HDPAD_R = crate::BitReader<HDPAD_A>;
#[doc = "User Flash hide protection area disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDPAD_A {
    #[doc = "0: User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    Enabled = 0,
    #[doc = "1: User Flash memory hide protection area disabled"]
    Disabled = 1,
}
impl From<HDPAD_A> for bool {
    #[inline(always)]
    fn from(variant: HDPAD_A) -> Self {
        variant as u8 != 0
    }
}
impl HDPAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDPAD_A {
        match self.bits {
            false => HDPAD_A::Enabled,
            true => HDPAD_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDPAD_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDPAD_A::Disabled
    }
}
#[doc = "Field `HDPAD` writer - User Flash hide protection area disabled"]
pub type HDPAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, HDPAD_A, O>;
impl<'a, const O: u8> HDPAD_W<'a, O> {
    #[doc = "User Flash memory hide protection area enabled. HDPSA\\[6:0\\]
contains the start address of the first 2-Kbyte page of the user Flash memory hide protection area"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDPAD_A::Enabled)
    }
    #[doc = "User Flash memory hide protection area disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDPAD_A::Disabled)
    }
}
#[doc = "Field `SUBGHSPISD` reader - sub-GHz radio SPI security disable"]
pub type SUBGHSPISD_R = crate::BitReader<SUBGHSPISD_A>;
#[doc = "sub-GHz radio SPI security disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHSPISD_A {
    #[doc = "0: sub-GHz radio SPI security enabled"]
    Enabled = 0,
    #[doc = "1: sub-GHz radio SPI security disabled"]
    Disabled = 1,
}
impl From<SUBGHSPISD_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHSPISD_A) -> Self {
        variant as u8 != 0
    }
}
impl SUBGHSPISD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUBGHSPISD_A {
        match self.bits {
            false => SUBGHSPISD_A::Enabled,
            true => SUBGHSPISD_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHSPISD_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHSPISD_A::Disabled
    }
}
#[doc = "Field `SUBGHSPISD` writer - sub-GHz radio SPI security disable"]
pub type SUBGHSPISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, SUBGHSPISD_A, O>;
impl<'a, const O: u8> SUBGHSPISD_W<'a, O> {
    #[doc = "sub-GHz radio SPI security enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUBGHSPISD_A::Enabled)
    }
    #[doc = "sub-GHz radio SPI security disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUBGHSPISD_A::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    pub fn hdpsa(&self) -> HDPSA_R {
        HDPSA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    pub fn hdpad(&self) -> HDPAD_R {
        HDPAD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    pub fn subghspisd(&self) -> SUBGHSPISD_R {
        SUBGHSPISD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Secure Flash start address"]
    #[inline(always)]
    #[must_use]
    pub fn sfsa(&mut self) -> SFSA_W<0> {
        SFSA_W::new(self)
    }
    #[doc = "Bit 7 - Flash security disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<7> {
        FSD_W::new(self)
    }
    #[doc = "Bit 12 - DDS"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<12> {
        DDS_W::new(self)
    }
    #[doc = "Bits 16:22 - User Flash hide protection area start address"]
    #[inline(always)]
    #[must_use]
    pub fn hdpsa(&mut self) -> HDPSA_W<16> {
        HDPSA_W::new(self)
    }
    #[doc = "Bit 23 - User Flash hide protection area disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hdpad(&mut self) -> HDPAD_W<23> {
        HDPAD_W::new(self)
    }
    #[doc = "Bit 31 - sub-GHz radio SPI security disable"]
    #[inline(always)]
    #[must_use]
    pub fn subghspisd(&mut self) -> SUBGHSPISD_W<31> {
        SUBGHSPISD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure Flash start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfr::R](R) reader structure"]
impl crate::Readable for SFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR to value 0xffff_efff"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_efff;
}
