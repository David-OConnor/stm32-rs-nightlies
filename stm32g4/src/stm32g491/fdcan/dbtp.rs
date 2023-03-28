#[doc = "Register `DBTP` reader"]
pub struct R(crate::R<DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBTP` writer"]
pub struct W(crate::W<DBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBTP_SPEC>;
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
impl From<crate::W<DBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSJW` reader - DSJW"]
pub type DSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSJW` writer - DSJW"]
pub type DSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTSEG2` reader - DTSEG2"]
pub type DTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEG2` writer - DTSEG2"]
pub type DTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTSEG1` writer - DTSEG1"]
pub type DTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBTP_SPEC, u8, u8, 5, O>;
#[doc = "Field `DBRP` reader - DBRP"]
pub type DBRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBRP` writer - DBRP"]
pub type DBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBTP_SPEC, u8, u8, 5, O>;
#[doc = "Field `TDC` reader - TDC"]
pub type TDC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<0> {
        DSJW_W::new(self)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<4> {
        DTSEG2_W::new(self)
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<8> {
        DTSEG1_W::new(self)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<16> {
        DBRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbtp](index.html) module"]
pub struct DBTP_SPEC;
impl crate::RegisterSpec for DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbtp::R](R) reader structure"]
impl crate::Readable for DBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbtp::W](W) writer structure"]
impl crate::Writable for DBTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a33;
}
