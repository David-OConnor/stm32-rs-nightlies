#[doc = "Register `ETH_MACA2HR` reader"]
pub struct R(crate::R<ETH_MACA2HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACA2HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACA2HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACA2HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACA2HR` writer"]
pub struct W(crate::W<ETH_MACA2HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACA2HR_SPEC>;
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
impl From<crate::W<ETH_MACA2HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACA2HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
pub type ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRHI` writer - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
pub type ADDRHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA2HR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MBC` reader - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBC` writer - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA2HR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SA` reader - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
pub type SA_R = crate::BitReader<bool>;
#[doc = "Field `SA` writer - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACA2HR_SPEC, bool, O>;
#[doc = "Field `AE` reader - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_R = crate::BitReader<bool>;
#[doc = "Field `AE` writer - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACA2HR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]
This field contains the upper 16 bits\\[47:32\\]
of the second 6-byte MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<0> {
        ADDRHI_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: Bit 29: ETH_MACAxHR\\[15:8\\]
Bit 28: ETH_MACAxHR\\[7:0\\]
Bit 27: ETH_MACAxLR\\[31:24\\]
Bit 26: ETH_MACAxLR\\[23:16\\]
Bit 25: ETH_MACAxLR\\[15:8\\]
Bit 24: ETH_MACAxLR\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Addressx\\[47:0\\]
is used to compare with the SA fields of the received packet. When this bit is reset, the MAC Address x\\[47:0\\]
is used to compare with the DA fields of the received packet."]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address 2 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maca2hr](index.html) module"]
pub struct ETH_MACA2HR_SPEC;
impl crate::RegisterSpec for ETH_MACA2HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maca2hr::R](R) reader structure"]
impl crate::Readable for ETH_MACA2HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maca2hr::W](W) writer structure"]
impl crate::Writable for ETH_MACA2HR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACA2HR to value 0xffff"]
impl crate::Resettable for ETH_MACA2HR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}