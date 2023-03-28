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
#[doc = "Field `SBRV` reader - cortex M0 access control register"]
pub type SBRV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SBRV` writer - cortex M0 access control register"]
pub type SBRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRRVR_SPEC, u32, u32, 18, O>;
#[doc = "Field `SBRSA` reader - Secure backup SRAM2a start address"]
pub type SBRSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBRSA` writer - Secure backup SRAM2a start address"]
pub type SBRSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRRVR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BRSD` reader - backup SRAM2a security disable"]
pub type BRSD_R = crate::BitReader<bool>;
#[doc = "Field `BRSD` writer - backup SRAM2a security disable"]
pub type BRSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, bool, O>;
#[doc = "Field `SNBRSA` reader - Secure non backup SRAM2a start address"]
pub type SNBRSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNBRSA` writer - Secure non backup SRAM2a start address"]
pub type SNBRSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRRVR_SPEC, u8, u8, 5, O>;
#[doc = "Field `NBRSD` reader - non-backup SRAM2b security disable"]
pub type NBRSD_R = crate::BitReader<bool>;
#[doc = "Field `NBRSD` writer - non-backup SRAM2b security disable"]
pub type NBRSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, bool, O>;
#[doc = "Field `C2OPT` reader - CPU2 cortex M0 boot reset vector memory selection"]
pub type C2OPT_R = crate::BitReader<bool>;
#[doc = "Field `C2OPT` writer - CPU2 cortex M0 boot reset vector memory selection"]
pub type C2OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRRVR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    #[must_use]
    pub fn sbrv(&mut self) -> SBRV_W<0> {
        SBRV_W::new(self)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsa(&mut self) -> SBRSA_W<18> {
        SBRSA_W::new(self)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    #[must_use]
    pub fn brsd(&mut self) -> BRSD_W<23> {
        BRSD_W::new(self)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    #[must_use]
    pub fn snbrsa(&mut self) -> SNBRSA_W<25> {
        SNBRSA_W::new(self)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbrsd(&mut self) -> NBRSD_W<30> {
        NBRSD_W::new(self)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
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
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srrvr](index.html) module"]
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
#[doc = "`reset()` method sets SRRVR to value 0x0100_0000"]
impl crate::Resettable for SRRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
