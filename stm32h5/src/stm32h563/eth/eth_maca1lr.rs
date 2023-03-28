#[doc = "Register `ETH_MACA1LR` reader"]
pub struct R(crate::R<ETH_MACA1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACA1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACA1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACA1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACA1LR` writer"]
pub struct W(crate::W<ETH_MACA1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACA1LR_SPEC>;
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
impl From<crate::W<ETH_MACA1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACA1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLO` reader - MAC Address x \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
pub type ADDRLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRLO` writer - MAC Address x \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
pub type ADDRLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA1LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Address x \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address x \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. The MAC uses this field for filtering the received packets and inserting the MAC address in the Transmit Flow Control (Pause) Packets."]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<0> {
        ADDRLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address 1 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca1lr](index.html) module"]
pub struct ETH_MACA1LR_SPEC;
impl crate::RegisterSpec for ETH_MACA1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maca1lr::R](R) reader structure"]
impl crate::Readable for ETH_MACA1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maca1lr::W](W) writer structure"]
impl crate::Writable for ETH_MACA1LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for ETH_MACA1LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
