#[doc = "Register `ETH_MMC_CONTROL` reader"]
pub struct R(crate::R<ETH_MMC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MMC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MMC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MMC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MMC_CONTROL` writer"]
pub struct W(crate::W<ETH_MMC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MMC_CONTROL_SPEC>;
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
impl From<crate::W<ETH_MMC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MMC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTRST` reader - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
pub type CNTRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTRST` writer - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
pub type CNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `CNTSTOPRO` reader - Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
pub type CNTSTOPRO_R = crate::BitReader<bool>;
#[doc = "Field `CNTSTOPRO` writer - Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
pub type CNTSTOPRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `RSTONRD` reader - Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
pub type RSTONRD_R = crate::BitReader<bool>;
#[doc = "Field `RSTONRD` writer - Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
pub type RSTONRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `CNTFREEZ` reader - MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received packet. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
pub type CNTFREEZ_R = crate::BitReader<bool>;
#[doc = "Field `CNTFREEZ` writer - MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received packet. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
pub type CNTFREEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `CNTPRST` reader - Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit. This bit is cleared automatically after 1 clock cycle. This bit, along with the CNTPRSTLVL bit, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
pub type CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRST` writer - Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit. This bit is cleared automatically after 1 clock cycle. This bit, along with the CNTPRSTLVL bit, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `CNTPRSTLVL` reader - Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (Half 2Kbytes) and all packet-counters get preset to 0x7FFF_FFF0 (Half 16). When this bit is high and the CNTPRST bit is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (Full 2Kbytes) and all packet-counters get preset to 0xFFFF_FFF0 (Full 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and packet counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
pub type CNTPRSTLVL_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRSTLVL` writer - Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (Half 2Kbytes) and all packet-counters get preset to 0x7FFF_FFF0 (Half 16). When this bit is high and the CNTPRST bit is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (Full 2Kbytes) and all packet-counters get preset to 0xFFFF_FFF0 (Full 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and packet counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
pub type CNTPRSTLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
#[doc = "Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Packets The CNTRST bit has a higher priority than the CNTPRST bit. Therefore, when the software tries to set both bits in the same write cycle, all counters are cleared and the CNTPRST bit is not set. When set, the MAC updates all related MMC Counters for Broadcast packets that are dropped because of the setting of the DBF bit of Packet filtering control register (ETH_MACPFR). When reset, the MMC Counters are not updated for dropped Broadcast packets."]
pub type UCDBC_R = crate::BitReader<bool>;
#[doc = "Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Packets The CNTRST bit has a higher priority than the CNTPRST bit. Therefore, when the software tries to set both bits in the same write cycle, all counters are cleared and the CNTPRST bit is not set. When set, the MAC updates all related MMC Counters for Broadcast packets that are dropped because of the setting of the DBF bit of Packet filtering control register (ETH_MACPFR). When reset, the MMC Counters are not updated for dropped Broadcast packets."]
pub type UCDBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MMC_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received packet. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit. This bit is cleared automatically after 1 clock cycle. This bit, along with the CNTPRSTLVL bit, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (Half 2Kbytes) and all packet-counters get preset to 0x7FFF_FFF0 (Half 16). When this bit is high and the CNTPRST bit is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (Full 2Kbytes) and all packet-counters get preset to 0xFFFF_FFF0 (Full 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and packet counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Packets The CNTRST bit has a higher priority than the CNTPRST bit. Therefore, when the software tries to set both bits in the same write cycle, all counters are cleared and the CNTPRST bit is not set. When set, the MAC updates all related MMC Counters for Broadcast packets that are dropped because of the setting of the DBF bit of Packet filtering control register (ETH_MACPFR). When reset, the MMC Counters are not updated for dropped Broadcast packets."]
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<0> {
        CNTRST_W::new(self)
    }
    #[doc = "Bit 1 - Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W<1> {
        CNTSTOPRO_W::new(self)
    }
    #[doc = "Bit 2 - Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
    #[inline(always)]
    #[must_use]
    pub fn rstonrd(&mut self) -> RSTONRD_W<2> {
        RSTONRD_W::new(self)
    }
    #[doc = "Bit 3 - MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received packet. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W<3> {
        CNTFREEZ_W::new(self)
    }
    #[doc = "Bit 4 - Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit. This bit is cleared automatically after 1 clock cycle. This bit, along with the CNTPRSTLVL bit, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<4> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 5 - Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (Half 2Kbytes) and all packet-counters get preset to 0x7FFF_FFF0 (Half 16). When this bit is high and the CNTPRST bit is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (Full 2Kbytes) and all packet-counters get preset to 0xFFFF_FFF0 (Full 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and packet counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
    #[inline(always)]
    #[must_use]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W<5> {
        CNTPRSTLVL_W::new(self)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Packets The CNTRST bit has a higher priority than the CNTPRST bit. Therefore, when the software tries to set both bits in the same write cycle, all counters are cleared and the CNTPRST bit is not set. When set, the MAC updates all related MMC Counters for Broadcast packets that are dropped because of the setting of the DBF bit of Packet filtering control register (ETH_MACPFR). When reset, the MMC Counters are not updated for dropped Broadcast packets."]
    #[inline(always)]
    #[must_use]
    pub fn ucdbc(&mut self) -> UCDBC_W<8> {
        UCDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mmc_control](index.html) module"]
pub struct ETH_MMC_CONTROL_SPEC;
impl crate::RegisterSpec for ETH_MMC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mmc_control::R](R) reader structure"]
impl crate::Readable for ETH_MMC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mmc_control::W](W) writer structure"]
impl crate::Writable for ETH_MMC_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MMC_CONTROL to value 0"]
impl crate::Resettable for ETH_MMC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
