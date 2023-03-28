#[doc = "Register `MASKR` reader"]
pub struct R(crate::R<MASKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASKR` writer"]
pub struct W(crate::W<MASKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASKR_SPEC>;
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
impl From<crate::W<MASKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable"]
pub type CTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable"]
pub type CTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable"]
pub type DTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable"]
pub type DTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable"]
pub type CMDRENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable"]
pub type CMDRENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable"]
pub type CMDSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable"]
pub type CMDSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable"]
pub type DATAENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable"]
pub type DATAENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DHOLDIE` reader - Data hold interrupt enable"]
pub type DHOLDIE_R = crate::BitReader<bool>;
#[doc = "Field `DHOLDIE` writer - Data hold interrupt enable"]
pub type DHOLDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable"]
pub type DBCKENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable"]
pub type DBCKENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `DABORTIE` reader - Data transfer aborted interrupt enable"]
pub type DABORTIE_R = crate::BitReader<bool>;
#[doc = "Field `DABORTIE` writer - Data transfer aborted interrupt enable"]
pub type DABORTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `BUSYD0ENDIE` reader - BUSYD0END interrupt enable"]
pub type BUSYD0ENDIE_R = crate::BitReader<bool>;
#[doc = "Field `BUSYD0ENDIE` writer - BUSYD0END interrupt enable"]
pub type BUSYD0ENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_R = crate::BitReader<bool>;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `ACKFAILIE` reader - Acknowledgment Fail interrupt enable"]
pub type ACKFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `ACKFAILIE` writer - Acknowledgment Fail interrupt enable"]
pub type ACKFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `ACKTIMEOUTIE` reader - Acknowledgment timeout interrupt enable"]
pub type ACKTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `ACKTIMEOUTIE` writer - Acknowledgment timeout interrupt enable"]
pub type ACKTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `VSWENDIE` reader - Voltage switch critical timing section completion interrupt enable"]
pub type VSWENDIE_R = crate::BitReader<bool>;
#[doc = "Field `VSWENDIE` writer - Voltage switch critical timing section completion interrupt enable"]
pub type VSWENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `CKSTOPIE` reader - CKSTOPIE"]
pub type CKSTOPIE_R = crate::BitReader<bool>;
#[doc = "Field `CKSTOPIE` writer - CKSTOPIE"]
pub type CKSTOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
#[doc = "Field `IDMABTCIE` reader - IDMABTCIE"]
pub type IDMABTCIE_R = crate::BitReader<bool>;
#[doc = "Field `IDMABTCIE` writer - IDMABTCIE"]
pub type IDMABTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data hold interrupt enable"]
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable"]
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable"]
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable"]
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable"]
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable"]
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CKSTOPIE"]
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMABTCIE"]
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<0> {
        CCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<1> {
        DCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<2> {
        CTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<3> {
        DTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<4> {
        TXUNDERRIE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<5> {
        RXOVERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<6> {
        CMDRENDIE_W::new(self)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<7> {
        CMDSENTIE_W::new(self)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DATAENDIE_W<8> {
        DATAENDIE_W::new(self)
    }
    #[doc = "Bit 9 - Data hold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dholdie(&mut self) -> DHOLDIE_W<9> {
        DHOLDIE_W::new(self)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<10> {
        DBCKENDIE_W::new(self)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dabortie(&mut self) -> DABORTIE_W<11> {
        DABORTIE_W::new(self)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<14> {
        TXFIFOHEIE_W::new(self)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<15> {
        RXFIFOHFIE_W::new(self)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<17> {
        RXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<18> {
        TXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W<21> {
        BUSYD0ENDIE_W::new(self)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<22> {
        SDIOITIE_W::new(self)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W<23> {
        ACKFAILIE_W::new(self)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W<24> {
        ACKTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vswendie(&mut self) -> VSWENDIE_W<25> {
        VSWENDIE_W::new(self)
    }
    #[doc = "Bit 26 - CKSTOPIE"]
    #[inline(always)]
    #[must_use]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W<26> {
        CKSTOPIE_W::new(self)
    }
    #[doc = "Bit 28 - IDMABTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W<28> {
        IDMABTCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maskr](index.html) module"]
pub struct MASKR_SPEC;
impl crate::RegisterSpec for MASKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maskr::R](R) reader structure"]
impl crate::Readable for MASKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maskr::W](W) writer structure"]
impl crate::Writable for MASKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASKR to value 0"]
impl crate::Resettable for MASKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
