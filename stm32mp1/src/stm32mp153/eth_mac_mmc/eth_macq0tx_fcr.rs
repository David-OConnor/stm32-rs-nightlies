#[doc = "Register `ETH_MACQ0TxFCR` reader"]
pub struct R(crate::R<ETH_MACQ0TX_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACQ0TX_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACQ0TX_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACQ0TX_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACQ0TxFCR` writer"]
pub struct W(crate::W<ETH_MACQ0TX_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACQ0TX_FCR_SPEC>;
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
impl From<crate::W<ETH_MACQ0TX_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACQ0TX_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB_BPA` reader - FCB_BPA"]
pub type FCB_BPA_R = crate::BitReader<bool>;
#[doc = "Field `FCB_BPA` writer - FCB_BPA"]
pub type FCB_BPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACQ0TX_FCR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - TFE"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - TFE"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACQ0TX_FCR_SPEC, bool, O>;
#[doc = "Field `PLT` reader - PLT"]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - PLT"]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACQ0TX_FCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DZPQ` reader - DZPQ"]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - DZPQ"]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACQ0TX_FCR_SPEC, bool, O>;
#[doc = "Field `PT` reader - PT"]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - PT"]
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACQ0TX_FCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macq0tx_fcr](index.html) module"]
pub struct ETH_MACQ0TX_FCR_SPEC;
impl crate::RegisterSpec for ETH_MACQ0TX_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macq0tx_fcr::R](R) reader structure"]
impl crate::Readable for ETH_MACQ0TX_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macq0tx_fcr::W](W) writer structure"]
impl crate::Writable for ETH_MACQ0TX_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACQ0TxFCR to value 0"]
impl crate::Resettable for ETH_MACQ0TX_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
