#[doc = "Register `TX_PAYSZR` reader"]
pub struct R(crate::R<TX_PAYSZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PAYSZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PAYSZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PAYSZR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PAYSZR` writer"]
pub struct W(crate::W<TX_PAYSZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PAYSZR_SPEC>;
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
impl From<crate::W<TX_PAYSZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PAYSZR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPAYSZ` reader - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
pub type TXPAYSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXPAYSZ` writer - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
pub type TXPAYSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_PAYSZR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
    #[inline(always)]
    #[must_use]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W<0> {
        TXPAYSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Tx payload size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_payszr](index.html) module"]
pub struct TX_PAYSZR_SPEC;
impl crate::RegisterSpec for TX_PAYSZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_payszr::R](R) reader structure"]
impl crate::Readable for TX_PAYSZR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_payszr::W](W) writer structure"]
impl crate::Writable for TX_PAYSZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PAYSZR to value 0"]
impl crate::Resettable for TX_PAYSZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
