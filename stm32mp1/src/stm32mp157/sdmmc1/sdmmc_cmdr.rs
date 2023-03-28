#[doc = "Register `SDMMC_CMDR` reader"]
pub struct R(crate::R<SDMMC_CMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_CMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_CMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_CMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_CMDR` writer"]
pub struct W(crate::W<SDMMC_CMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_CMDR_SPEC>;
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
impl From<crate::W<SDMMC_CMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_CMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - CMDINDEX"]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - CMDINDEX"]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_CMDR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDTRANS` reader - CMDTRANS"]
pub type CMDTRANS_R = crate::BitReader<bool>;
#[doc = "Field `CMDTRANS` writer - CMDTRANS"]
pub type CMDTRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSTOP` reader - CMDSTOP"]
pub type CMDSTOP_R = crate::BitReader<bool>;
#[doc = "Field `CMDSTOP` writer - CMDSTOP"]
pub type CMDSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `WAITRESP` reader - WAITRESP"]
pub type WAITRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITRESP` writer - WAITRESP"]
pub type WAITRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_CMDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAITINT` reader - WAITINT"]
pub type WAITINT_R = crate::BitReader<bool>;
#[doc = "Field `WAITINT` writer - WAITINT"]
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `WAITPEND` reader - WAITPEND"]
pub type WAITPEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITPEND` writer - WAITPEND"]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CPSMEN` reader - CPSMEN"]
pub type CPSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CPSMEN` writer - CPSMEN"]
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `DTHOLD` reader - DTHOLD"]
pub type DTHOLD_R = crate::BitReader<bool>;
#[doc = "Field `DTHOLD` writer - DTHOLD"]
pub type DTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTMODE` reader - BOOTMODE"]
pub type BOOTMODE_R = crate::BitReader<bool>;
#[doc = "Field `BOOTMODE` writer - BOOTMODE"]
pub type BOOTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTEN` reader - BOOTEN"]
pub type BOOTEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTEN` writer - BOOTEN"]
pub type BOOTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSUSPEND` reader - CMDSUSPEND"]
pub type CMDSUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `CMDSUSPEND` writer - CMDSUSPEND"]
pub type CMDSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<6> {
        CMDTRANS_W::new(self)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<7> {
        CMDSTOP_W::new(self)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<8> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<10> {
        WAITINT_W::new(self)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<11> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<12> {
        CPSMEN_W::new(self)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn dthold(&mut self) -> DTHOLD_W<13> {
        DTHOLD_W::new(self)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BOOTMODE_W<14> {
        BOOTMODE_W::new(self)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<15> {
        BOOTEN_W::new(self)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W<16> {
        CMDSUSPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_cmdr](index.html) module"]
pub struct SDMMC_CMDR_SPEC;
impl crate::RegisterSpec for SDMMC_CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_cmdr::R](R) reader structure"]
impl crate::Readable for SDMMC_CMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_cmdr::W](W) writer structure"]
impl crate::Writable for SDMMC_CMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_CMDR to value 0"]
impl crate::Resettable for SDMMC_CMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
