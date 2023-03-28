#[doc = "Register `CMDR` reader"]
pub struct R(crate::R<CMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDR` writer"]
pub struct W(crate::W<CMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDR_SPEC>;
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
impl From<crate::W<CMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - Command index"]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - Command index"]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDTRANS` reader - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
pub type CMDTRANS_R = crate::BitReader<bool>;
#[doc = "Field `CMDTRANS` writer - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
pub type CMDTRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSTOP` reader - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
pub type CMDSTOP_R = crate::BitReader<bool>;
#[doc = "Field `CMDSTOP` writer - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
pub type CMDSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `WAITRESP` reader - Wait for response bits"]
pub type WAITRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITRESP` writer - Wait for response bits"]
pub type WAITRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request"]
pub type WAITINT_R = crate::BitReader<bool>;
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request"]
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
pub type WAITPEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `DTHOLD` reader - Hold new data block transmission and reception in the DPSM"]
pub type DTHOLD_R = crate::BitReader<bool>;
#[doc = "Field `DTHOLD` writer - Hold new data block transmission and reception in the DPSM"]
pub type DTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTMODE` reader - Select the boot mode procedure to be used"]
pub type BOOTMODE_R = crate::BitReader<bool>;
#[doc = "Field `BOOTMODE` writer - Select the boot mode procedure to be used"]
pub type BOOTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTEN` reader - Enable boot mode procedure"]
pub type BOOTEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTEN` writer - Enable boot mode procedure"]
pub type BOOTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSUSPEND` reader - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
pub type CMDSUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `CMDSUSPEND` writer - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
pub type CMDSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable boot mode procedure"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<6> {
        CMDTRANS_W::new(self)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<7> {
        CMDSTOP_W::new(self)
    }
    #[doc = "Bits 8:9 - Wait for response bits"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<8> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<10> {
        WAITINT_W::new(self)
    }
    #[doc = "Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<11> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<12> {
        CPSMEN_W::new(self)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn dthold(&mut self) -> DTHOLD_W<13> {
        DTHOLD_W::new(self)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BOOTMODE_W<14> {
        BOOTMODE_W::new(self)
    }
    #[doc = "Bit 15 - Enable boot mode procedure"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<15> {
        BOOTEN_W::new(self)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
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
#[doc = "command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](index.html) module"]
pub struct CMDR_SPEC;
impl crate::RegisterSpec for CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdr::R](R) reader structure"]
impl crate::Readable for CMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdr::W](W) writer structure"]
impl crate::Writable for CMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
