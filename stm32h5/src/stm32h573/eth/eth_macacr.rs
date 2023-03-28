#[doc = "Register `ETH_MACACR` reader"]
pub struct R(crate::R<ETH_MACACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACACR` writer"]
pub struct W(crate::W<ETH_MACACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACACR_SPEC>;
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
impl From<crate::W<ETH_MACACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
pub type ATSFC_R = crate::BitReader<bool>;
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
pub type ATSFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
#[doc = "Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN0_R = crate::BitReader<bool>;
#[doc = "Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
#[doc = "Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN1_R = crate::BitReader<bool>;
#[doc = "Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
#[doc = "Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN2_R = crate::BitReader<bool>;
#[doc = "Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
#[doc = "Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN3_R = crate::BitReader<bool>;
#[doc = "Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn atsfc(&mut self) -> ATSFC_W<0> {
        ATSFC_W::new(self)
    }
    #[doc = "Bit 4 - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen0(&mut self) -> ATSEN0_W<4> {
        ATSEN0_W::new(self)
    }
    #[doc = "Bit 5 - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen1(&mut self) -> ATSEN1_W<5> {
        ATSEN1_W::new(self)
    }
    #[doc = "Bit 6 - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen2(&mut self) -> ATSEN2_W<6> {
        ATSEN2_W::new(self)
    }
    #[doc = "Bit 7 - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen3(&mut self) -> ATSEN3_W<7> {
        ATSEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macacr](index.html) module"]
pub struct ETH_MACACR_SPEC;
impl crate::RegisterSpec for ETH_MACACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macacr::R](R) reader structure"]
impl crate::Readable for ETH_MACACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macacr::W](W) writer structure"]
impl crate::Writable for ETH_MACACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACACR to value 0"]
impl crate::Resettable for ETH_MACACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
