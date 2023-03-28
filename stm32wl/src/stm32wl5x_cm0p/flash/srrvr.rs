#[doc = "Register `SRRVR` reader"]
pub struct R(crate::R<SRRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRRVR` writer"]
pub struct W(crate::W<SRRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRRVR_SPEC>;
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
impl From<crate::W<SRRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBRV` reader - CPU2 boot reset vector"]
pub type SBRV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SBRV` writer - CPU2 boot reset vector"]
pub type SBRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRRVR_SPEC, u16, u16, 16, O>;
#[doc = "Field `SBRSA` reader - Secure backup SRAM2 start address"]
pub type SBRSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBRSA` writer - Secure backup SRAM2 start address"]
pub type SBRSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRRVR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BRSD` reader - backup SRAM2 security disable"]
pub type BRSD_R = crate::BitReader<BRSD_A>;
#[doc = "backup SRAM2 security disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSD_A {
    #[doc = "0: SRAM2 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area"]
    Secure = 0,
    #[doc = "1: SRAM2 is non-secure"]
    NonSecure = 1,
}
impl From<BRSD_A> for bool {
    #[inline(always)]
    fn from(variant: BRSD_A) -> Self {
        variant as u8 != 0
    }
}
impl BRSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSD_A {
        match self.bits {
            false => BRSD_A::Secure,
            true => BRSD_A::NonSecure,
        }
    }
    #[doc = "Checks if the value of the field is `Secure`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == BRSD_A::Secure
    }
    #[doc = "Checks if the value of the field is `NonSecure`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == BRSD_A::NonSecure
    }
}
#[doc = "Field `BRSD` writer - backup SRAM2 security disable"]
pub type BRSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, BRSD_A, O>;
impl<'a, const O: u8> BRSD_W<'a, O> {
    #[doc = "SRAM2 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(BRSD_A::Secure)
    }
    #[doc = "SRAM2 is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(BRSD_A::NonSecure)
    }
}
#[doc = "Field `SNBRSA` reader - Secure non-backup SRAM1 start address"]
pub type SNBRSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNBRSA` writer - Secure non-backup SRAM1 start address"]
pub type SNBRSA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRRVR_SPEC, u8, u8, 5, O>;
#[doc = "Field `NBRSD` reader - NBRSD"]
pub type NBRSD_R = crate::BitReader<NBRSD_A>;
#[doc = "NBRSD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBRSD_A {
    #[doc = "0: SRAM1 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area"]
    Secure = 0,
    #[doc = "1: SRAM1 is non-secure"]
    NonSecure = 1,
}
impl From<NBRSD_A> for bool {
    #[inline(always)]
    fn from(variant: NBRSD_A) -> Self {
        variant as u8 != 0
    }
}
impl NBRSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBRSD_A {
        match self.bits {
            false => NBRSD_A::Secure,
            true => NBRSD_A::NonSecure,
        }
    }
    #[doc = "Checks if the value of the field is `Secure`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == NBRSD_A::Secure
    }
    #[doc = "Checks if the value of the field is `NonSecure`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == NBRSD_A::NonSecure
    }
}
#[doc = "Field `NBRSD` writer - NBRSD"]
pub type NBRSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, NBRSD_A, O>;
impl<'a, const O: u8> NBRSD_W<'a, O> {
    #[doc = "SRAM1 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(NBRSD_A::Secure)
    }
    #[doc = "SRAM1 is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(NBRSD_A::NonSecure)
    }
}
#[doc = "Field `C2OPT` reader - C2OPT"]
pub type C2OPT_R = crate::BitReader<C2OPT_A>;
#[doc = "C2OPT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2OPT_A {
    #[doc = "0: SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV"]
    Sram = 0,
    #[doc = "1: SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV"]
    Flash = 1,
}
impl From<C2OPT_A> for bool {
    #[inline(always)]
    fn from(variant: C2OPT_A) -> Self {
        variant as u8 != 0
    }
}
impl C2OPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C2OPT_A {
        match self.bits {
            false => C2OPT_A::Sram,
            true => C2OPT_A::Flash,
        }
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == C2OPT_A::Sram
    }
    #[doc = "Checks if the value of the field is `Flash`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == C2OPT_A::Flash
    }
}
#[doc = "Field `C2OPT` writer - C2OPT"]
pub type C2OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, C2OPT_A, O>;
impl<'a, const O: u8> C2OPT_W<'a, O> {
    #[doc = "SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(C2OPT_A::Sram)
    }
    #[doc = "SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(C2OPT_A::Flash)
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU2 boot reset vector"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2 start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2 security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Secure non-backup SRAM1 start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - NBRSD"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - C2OPT"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU2 boot reset vector"]
    #[inline(always)]
    #[must_use]
    pub fn sbrv(&mut self) -> SBRV_W<0> {
        SBRV_W::new(self)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2 start address"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsa(&mut self) -> SBRSA_W<18> {
        SBRSA_W::new(self)
    }
    #[doc = "Bit 23 - backup SRAM2 security disable"]
    #[inline(always)]
    #[must_use]
    pub fn brsd(&mut self) -> BRSD_W<23> {
        BRSD_W::new(self)
    }
    #[doc = "Bits 25:29 - Secure non-backup SRAM1 start address"]
    #[inline(always)]
    #[must_use]
    pub fn snbrsa(&mut self) -> SNBRSA_W<25> {
        SNBRSA_W::new(self)
    }
    #[doc = "Bit 30 - NBRSD"]
    #[inline(always)]
    #[must_use]
    pub fn nbrsd(&mut self) -> NBRSD_W<30> {
        NBRSD_W::new(self)
    }
    #[doc = "Bit 31 - C2OPT"]
    #[inline(always)]
    #[must_use]
    pub fn c2opt(&mut self) -> C2OPT_W<31> {
        C2OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure SRAM start address and CPU2 reset vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srrvr](index.html) module"]
pub struct SRRVR_SPEC;
impl crate::RegisterSpec for SRRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srrvr::R](R) reader structure"]
impl crate::Readable for SRRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srrvr::W](W) writer structure"]
impl crate::Writable for SRRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRRVR to value 0xffff_8000"]
impl crate::Resettable for SRRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_8000;
}
