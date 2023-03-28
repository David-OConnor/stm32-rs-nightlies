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
#[doc = "Field `CMDINDEX` reader - Command index This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - Command index This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_CMDR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMDTRANS` reader - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
pub type CMDTRANS_R = crate::BitReader<bool>;
#[doc = "Field `CMDTRANS` writer - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
pub type CMDTRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSTOP` reader - The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the abort signal to the DPSM when the command is sent."]
pub type CMDSTOP_R = crate::BitReader<bool>;
#[doc = "Field `CMDSTOP` writer - The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the abort signal to the DPSM when the command is sent."]
pub type CMDSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `WAITRESP` reader - Wait for response bits This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WAITRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITRESP` writer - Wait for response bits This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WAITRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_CMDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, it causes the abort of the interrupt mode."]
pub type WAITINT_R = crate::BitReader<bool>;
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, it causes the abort of the interrupt mode."]
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `WAITPEND` reader - CPSM waits for end of data transfer (CmdPend internal signal) from DPSM This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = e•MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
pub type WAITPEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITPEND` writer - CPSM waits for end of data transfer (CmdPend internal signal) from DPSM This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = e•MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command is transfered nor boot procedure is started. CPSMEN is cleared to 0. During Read Wait with SDMMC_CK stopped no command is sent and CPSMEN is kept 0."]
pub type CPSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command is transfered nor boot procedure is started. CPSMEN is cleared to 0. During Read Wait with SDMMC_CK stopped no command is sent and CPSMEN is kept 0."]
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `DTHOLD` reader - Hold new data block transmission and reception in the DPSM If this bit is set, the DPSM does not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
pub type DTHOLD_R = crate::BitReader<bool>;
#[doc = "Field `DTHOLD` writer - Hold new data block transmission and reception in the DPSM If this bit is set, the DPSM does not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
pub type DTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTMODE` reader - Select the boot mode procedure to be used This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
pub type BOOTMODE_R = crate::BitReader<bool>;
#[doc = "Field `BOOTMODE` writer - Select the boot mode procedure to be used This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
pub type BOOTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `BOOTEN` reader - Enable boot mode procedure"]
pub type BOOTEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTEN` writer - Enable boot mode procedure"]
pub type BOOTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
#[doc = "Field `CMDSUSPEND` reader - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
pub type CMDSUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `CMDSUSPEND` writer - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
pub type CMDSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_CMDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Command index This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the abort signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Wait for response bits This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, it causes the abort of the interrupt mode."]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPSM waits for end of data transfer (CmdPend internal signal) from DPSM This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = e•MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command is transfered nor boot procedure is started. CPSMEN is cleared to 0. During Read Wait with SDMMC_CK stopped no command is sent and CPSMEN is kept 0."]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM If this bit is set, the DPSM does not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable boot mode procedure"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<6> {
        CMDTRANS_W::new(self)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the abort signal to the DPSM when the command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<7> {
        CMDSTOP_W::new(self)
    }
    #[doc = "Bits 8:9 - Wait for response bits This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<8> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, it causes the abort of the interrupt mode."]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<10> {
        WAITINT_W::new(self)
    }
    #[doc = "Bit 11 - CPSM waits for end of data transfer (CmdPend internal signal) from DPSM This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = e•MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<11> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command is transfered nor boot procedure is started. CPSMEN is cleared to 0. During Read Wait with SDMMC_CK stopped no command is sent and CPSMEN is kept 0."]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<12> {
        CPSMEN_W::new(self)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM If this bit is set, the DPSM does not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
    #[inline(always)]
    #[must_use]
    pub fn dthold(&mut self) -> DTHOLD_W<13> {
        DTHOLD_W::new(self)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
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
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
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
#[doc = "SDMMC command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_cmdr](index.html) module"]
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
