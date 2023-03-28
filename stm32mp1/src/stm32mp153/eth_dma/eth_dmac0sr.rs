#[doc = "Register `ETH_DMAC0SR` reader"]
pub struct R(crate::R<ETH_DMAC0SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0SR` writer"]
pub struct W(crate::W<ETH_DMAC0SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0SR_SPEC>;
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
impl From<crate::W<ETH_DMAC0SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader<bool>;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable"]
pub type TBU_R = crate::BitReader<bool>;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable"]
pub type TBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable"]
pub type RBU_R = crate::BitReader<bool>;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable"]
pub type RBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RPS_R = crate::BitReader<bool>;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader<bool>;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type ETI_R = crate::BitReader<bool>;
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub type ETI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type ERI_R = crate::BitReader<bool>;
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub type ERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error"]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal Bus Error"]
pub type FBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `CDE` reader - Context Descriptor Error"]
pub type CDE_R = crate::BitReader<bool>;
#[doc = "Field `CDE` writer - Context Descriptor Error"]
pub type CDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SR_SPEC, bool, O>;
#[doc = "Field `TEB` reader - Tx DMA Error Bits"]
pub type TEB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEB` writer - Tx DMA Error Bits"]
pub type TEB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0SR_SPEC, u8, u8, 3, O>;
#[doc = "Field `REB` reader - Rx DMA Error Bits"]
pub type REB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REB` writer - Rx DMA Error Bits"]
pub type REB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0SR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<2> {
        TBU_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<7> {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<10> {
        ETI_W::new(self)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<11> {
        ERI_W::new(self)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<12> {
        FBE_W::new(self)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CDE_W<13> {
        CDE_W::new(self)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<14> {
        AIS_W::new(self)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<15> {
        NIS_W::new(self)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    #[must_use]
    pub fn teb(&mut self) -> TEB_W<16> {
        TEB_W::new(self)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    #[must_use]
    pub fn reb(&mut self) -> REB_W<19> {
        REB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0sr](index.html) module"]
pub struct ETH_DMAC0SR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0sr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0sr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0SR to value 0"]
impl crate::Resettable for ETH_DMAC0SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
