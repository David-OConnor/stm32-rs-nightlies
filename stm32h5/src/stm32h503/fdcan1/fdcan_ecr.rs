#[doc = "Register `FDCAN_ECR` reader"]
pub struct R(crate::R<FDCAN_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_ECR` writer"]
pub struct W(crate::W<FDCAN_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_ECR_SPEC>;
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
impl From<crate::W<FDCAN_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEC` reader - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type TEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC` reader - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RP` reader - Receive error passive"]
pub type RP_R = crate::BitReader<bool>;
#[doc = "Field `CEL` reader - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEL` writer - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_ECR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<16> {
        CEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN error counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ecr](index.html) module"]
pub struct FDCAN_ECR_SPEC;
impl crate::RegisterSpec for FDCAN_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ecr::R](R) reader structure"]
impl crate::Readable for FDCAN_ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ecr::W](W) writer structure"]
impl crate::Writable for FDCAN_ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FDCAN_ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
